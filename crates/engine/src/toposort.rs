//! toposort.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   toposort.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     11/01/21   (Build 5.2.0)
//   Author:   L. Rossman
//
//   Topological sorting of conveyance network links
//-----------------------------------------------------------------------------

use crate::prelude::*;

//-----------------------------------------------------------------------------
//  Constants
//-----------------------------------------------------------------------------
// enum AdjListType {UNDIRECTED, DIRECTED};    // type of nodal adjacency list
const UNDIRECTED: i32 = 0;
const DIRECTED: i32 = 1;

/// The file-scope variables of toposort.c.
#[derive(Default)]
pub struct ToposortState {
    InDegree: Vec<i32>,                // number of incoming links to each node
    StartPos: Vec<i32>,                // start of a node's outlinks in AdjList
    AdjList: Vec<i32>,                 // list of outlink indexes for each node
    Stack: Vec<i32>,                   // array of nodes "reached" during sorting
    First: i32,                        // position of first node in stack
    Last: i32,                         // position of last node added to stack

    Examined: Vec<i8>,                 // TRUE if node included in spanning tree
    InTree: Vec<i8>,                   // state of each link in spanning tree:
                                       // 0 = unexamined,
                                       // 1 = in spanning tree,
                                       // 2 = chord of spanning tree
    LoopLinks: Vec<i32>,               // list of links which forms a loop
    LoopLinksLast: i32,                // number of links in a loop
}

//=============================================================================

/// Called by: routing.
pub fn toposort_sortLinks(prj: &mut Project, sortedLinks: &mut [i32]) {
//
//  Input:   none
//  Output:  sortedLinks = array of link indexes in sorted order
//  Purpose: sorts links from upstream to downstream.
//
    let mut i: i32;
    let mut n: i32 = 0;

    // --- no need to sort links for Dyn. Wave routing
    i = 0;
    while i < prj.Nobjects[LINK as usize] {
        sortedLinks[i as usize] = i;
        i += 1;
    }
    if prj.RouteModel == DW {
        // --- check for nodes with both incoming and outgoing
        //     dummy links (creates ambiguous ordering)
        checkDummyLinks(prj);
        if prj.ErrorCode != 0 {
            return;
        }

        // --- find number of outflow links for each node
        for i in 0..prj.Nobjects[NODE as usize] {
            prj.Node[i as usize].degree = 0;
        }
        for i in 0..prj.Nobjects[LINK as usize] {
            // --- if upstream node is an outfall, then increment outflow
            //     count for downstream node, otherwise increment count
            //     for upstream node
            n = prj.Link[i as usize].node1;
            if prj.Link[i as usize].direction < 0 {
                n = prj.Link[i as usize].node2;
            }
            if prj.Node[n as usize].type_ == OUTFALL {
                if prj.Link[i as usize].direction < 0 {
                    n = prj.Link[i as usize].node1;
                } else {
                    n = prj.Link[i as usize].node2;
                }
                prj.Node[n as usize].degree += 1;
            } else {
                prj.Node[n as usize].degree += 1;
            }
        }
        return;
    }

    // --- allocate arrays used for topo sorting
    if prj.ErrorCode != 0 {
        return;
    }
    let nNodes = prj.Nobjects[NODE as usize] as usize;
    let nLinks = prj.Nobjects[LINK as usize] as usize;
    prj.toposort.InDegree = vec![0; nNodes];
    prj.toposort.StartPos = vec![0; nNodes];
    prj.toposort.AdjList = vec![0; nLinks];
    prj.toposort.Stack = vec![0; nNodes];
    // (the out-of-memory branch of the C code is not needed)
    {
        // --- create a directed adjacency list of links leaving each node
        createAdjList(prj, DIRECTED);

        // --- adjust adjacency list for DIVIDER nodes
        adjustAdjList(prj);

        // --- find number of links entering each node
        for i in 0..prj.Nobjects[NODE as usize] {
            prj.toposort.InDegree[i as usize] = 0;
        }
        for i in 0..prj.Nobjects[LINK as usize] {
            let n2 = prj.Link[i as usize].node2;
            prj.toposort.InDegree[n2 as usize] += 1;
        }

        // --- topo sort the links
        n = topoSort(prj, sortedLinks);
    }

    // --- free allocated memory
    prj.toposort.InDegree = Vec::new();
    prj.toposort.StartPos = Vec::new();
    prj.toposort.AdjList = Vec::new();
    prj.toposort.Stack = Vec::new();

    // --- check that all links are included in SortedLinks
    if prj.ErrorCode == 0 && n != prj.Nobjects[LINK as usize] {
        report_writeErrorMsg(prj, ERR_LOOP, b"");
        findCycles(prj);
    }
}

//=============================================================================

fn createAdjList(prj: &mut Project, listType: i32) {
//
//  Input:   lsitType = DIRECTED or UNDIRECTED
//  Output:  none
//  Purpose: creates listing of links incident on each node.
//
    let mut i: i32;
    let mut k: i32;

    // --- determine degree of each node
    //     (for DIRECTED list only count link at its upstream node;
    //      for UNDIRECTED list count link at both end nodes)
    for i in 0..prj.Nobjects[NODE as usize] {
        prj.Node[i as usize].degree = 0;
    }
    for j in 0..prj.Nobjects[LINK as usize] {
        let n1 = prj.Link[j as usize].node1;
        prj.Node[n1 as usize].degree += 1;
        if listType == UNDIRECTED {
            let n2 = prj.Link[j as usize].node2;
            prj.Node[n2 as usize].degree += 1;
        }
    }

    // --- determine start position of each node in the adjacency list
    //     (the adjacency list, AdjList, is one long vector containing
    //      the individual node lists one after the other)
    prj.toposort.StartPos[0] = 0;
    i = 0;
    while i < prj.Nobjects[NODE as usize] - 1 {
        prj.toposort.StartPos[(i + 1) as usize] =
            prj.toposort.StartPos[i as usize] + prj.Node[i as usize].degree;
        prj.Node[i as usize].degree = 0;
        i += 1;
    }
    let last = (prj.Nobjects[NODE as usize] - 1) as usize;
    prj.Node[last].degree = 0;

    // --- traverse the list of links once more,
    //     adding each link's index to the proper
    //     position in the adjacency list
    for j in 0..prj.Nobjects[LINK as usize] {
        i = prj.Link[j as usize].node1;
        k = prj.toposort.StartPos[i as usize] + prj.Node[i as usize].degree;
        prj.toposort.AdjList[k as usize] = j;
        prj.Node[i as usize].degree += 1;
        if listType == UNDIRECTED {
            i = prj.Link[j as usize].node2;
            k = prj.toposort.StartPos[i as usize] + prj.Node[i as usize].degree;
            prj.toposort.AdjList[k as usize] = j;
            prj.Node[i as usize].degree += 1;
        }
    }
}

//=============================================================================

fn adjustAdjList(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: adjusts adjacency list for Divider nodes so that non-
//           diversion link appears before diversion link.
//
    let mut j: i32;
    let mut k: i32;
    let mut m: i32;

    // --- check each node
    for i in 0..prj.Nobjects[NODE as usize] {
        // --- skip nodes that are not Dividers
        if prj.Node[i as usize].type_ != DIVIDER {
            continue;
        }
        if prj.Node[i as usize].degree != 2 {
            continue;
        }

        // --- switch position of outgoing links at the node if the
        //     diversion link appears first in the adjacency list
        k = prj.Node[i as usize].subIndex;
        m = prj.toposort.StartPos[i as usize];
        j = prj.toposort.AdjList[m as usize];
        if j == prj.Divider[k as usize].link {
            prj.toposort.AdjList[m as usize] = prj.toposort.AdjList[(m + 1) as usize];
            prj.toposort.AdjList[(m + 1) as usize] = j;
        }
    }
}

//=============================================================================

fn topoSort(prj: &mut Project, sortedLinks: &mut [i32]) -> i32 {
//
//  Input:   none
//  Output:  sortedLinks = array of sorted link indexes,
//           returns number of links successfully sorted
//  Purpose: performs a stack-based topo sort of the drainage network's links.
//
    let mut j: i32;
    let mut n: i32;
    let mut i1: i32;
    let mut i2: i32;
    let mut k1: i32;
    let mut k2: i32;

    // --- initialize a stack which contains nodes with zero in-degree
    prj.toposort.First = 0;
    prj.toposort.Last = -1;
    for i in 0..prj.Nobjects[NODE as usize] {
        if prj.toposort.InDegree[i as usize] == 0 {
            prj.toposort.Last += 1;
            let last = prj.toposort.Last as usize;
            prj.toposort.Stack[last] = i;
        }
    }

    // --- traverse the stack, adding each node's outgoing link indexes
    //     to the SortedLinks array in the order processed
    n = 0;
    while prj.toposort.First <= prj.toposort.Last {
        // --- determine range of adjacency list indexes belonging to
        //     first node remaining on the stack
        i1 = prj.toposort.Stack[prj.toposort.First as usize];
        k1 = prj.toposort.StartPos[i1 as usize];
        k2 = k1 + prj.Node[i1 as usize].degree;

        // --- for each outgoing link from first node on stack
        let mut k = k1;
        while k < k2 {
            // --- add link index to current position in SortedLinks
            j = prj.toposort.AdjList[k as usize];
            sortedLinks[n as usize] = j;
            n += 1;

            // --- reduce in-degree of link's downstream node
            i2 = prj.Link[j as usize].node2;
            prj.toposort.InDegree[i2 as usize] -= 1;
            // --- add downstream node to stack if its in-degree is zero
            if prj.toposort.InDegree[i2 as usize] == 0 {
                prj.toposort.Last += 1;
                let last = prj.toposort.Last as usize;
                prj.toposort.Stack[last] = i2;
            }
            k += 1;
        }
        prj.toposort.First += 1;
    }
    n
}

//=============================================================================

fn findCycles(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: finds all cycles in the drainage network (i.e., closed loops that
//           start and end at the same node).
//
    let nNodes = prj.Nobjects[NODE as usize] as usize;
    let nLinks = prj.Nobjects[LINK as usize] as usize;

    // --- allocate arrays
    prj.toposort.AdjList = vec![0; 2 * nLinks];
    prj.toposort.StartPos = vec![0; nNodes];
    prj.toposort.Stack = vec![0; nNodes];
    prj.toposort.Examined = vec![0; nNodes];
    prj.toposort.InTree = vec![0; nLinks];
    prj.toposort.LoopLinks = vec![0; nLinks];
    {
        // --- create an undirected adjacency list for the nodes
        createAdjList(prj, UNDIRECTED);

        // --- set to empty the list of nodes examined and the list
        //     of links in the spanning tree
        for i in 0..prj.Nobjects[NODE as usize] {
            prj.toposort.Examined[i as usize] = 0;
        }
        for i in 0..prj.Nobjects[LINK as usize] {
            prj.toposort.InTree[i as usize] = 0;
        }

        // --- find a spanning tree for each unexamined node
        //     (cycles are identified as tree is constructed)
        for i in 0..prj.Nobjects[NODE as usize] {
            if prj.toposort.Examined[i as usize] != 0 {
                continue;
            }
            prj.toposort.Last = -1;
            findSpanningTree(prj, i);
        }
    }
    prj.toposort.StartPos = Vec::new();
    prj.toposort.AdjList = Vec::new();
    prj.toposort.Stack = Vec::new();
    prj.toposort.Examined = Vec::new();
    prj.toposort.InTree = Vec::new();
    prj.toposort.LoopLinks = Vec::new();
}

//=============================================================================

fn findSpanningTree(prj: &mut Project, startNode: i32) {
//
//  Input:   i = index of starting node of tree
//  Output:  none
//  Purpose: finds continuation of network's spanning tree of links.
//
    let mut nextNode: i32;
    let mut j: i32;
    let mut k: i32;
    let mut m: i32;
    // C ends with the tail call findSpanningTree(nextNode), which GCC -O2
    // compiles to a jump; it is written here as a loop so that the stack
    // does not grow with the number of nodes. The order of work is the same.
    let mut startNode = startNode;
    loop {

    // --- examine each link connected to node i
    m = prj.toposort.StartPos[startNode as usize];
    while m < prj.toposort.StartPos[startNode as usize] + prj.Node[startNode as usize].degree {
        // --- find which node (j) connects link k from start node
        k = prj.toposort.AdjList[m as usize];
        if prj.Link[k as usize].node1 == startNode {
            j = prj.Link[k as usize].node2;
        } else {
            j = prj.Link[k as usize].node1;
        }

        // --- if link k is not in the tree
        if prj.toposort.InTree[k as usize] == 0 {
            // --- if connecting node already examined,
            //     then link k forms a loop; mark it as a chord
            //     and check if loop forms a cycle
            if prj.toposort.Examined[j as usize] != 0 {
                prj.toposort.InTree[k as usize] = 2;
                evalLoop(prj, k);
            }
            // --- otherwise mark connected node as being examined,
            //     add it to the node stack, and mark the connecting
            //     link as being in the spanning tree
            else {
                prj.toposort.Examined[j as usize] = 1;
                prj.toposort.Last += 1;
                let last = prj.toposort.Last as usize;
                prj.toposort.Stack[last] = j;
                prj.toposort.InTree[k as usize] = 1;
            }
        }
        m += 1;
    }

    // --- continue to grow the spanning tree from
    //     the last node added to the stack
    if prj.toposort.Last >= 0 {
        nextNode = prj.toposort.Stack[prj.toposort.Last as usize];
        prj.toposort.Last -= 1;
        startNode = nextNode;           // findSpanningTree(nextNode);
    } else {
        break;
    }
    }
}

//=============================================================================

fn evalLoop(prj: &mut Project, startLink: i32) {
//
//  Input:   startLink = index of starting link of a loop
//  Output:  none
//  Purpose: checks if loop starting with a given link forms a closed cycle.
//
    let mut i: i32;                    // loop list index
    let mut i1: i32;                   // start & end node indexes
    let mut i2: i32;
    let mut j: i32;                    // index of link in loop
    let mut kount: i32;                // items per line counter
    let mut isCycle: i32;              // TRUE if loop forms a cycle

    // --- make startLink the first link in the loop
    prj.toposort.LoopLinksLast = 0;
    prj.toposort.LoopLinks[0] = startLink;

    // --- trace a path on the spanning tree that starts at the
    //     tail node of startLink and ends at its head node
    i1 = prj.Link[startLink as usize].node1;
    i2 = prj.Link[startLink as usize].node2;
    if traceLoop(prj, i1, i2, startLink) == 0 {
        return;
    }

    // --- check if all links on the path are oriented head-to-tail
    isCycle = TRUE;
    j = prj.toposort.LoopLinks[0];
    i2 = prj.Link[j as usize].node2;
    i = 1;
    while i <= prj.toposort.LoopLinksLast {
        j = prj.toposort.LoopLinks[i as usize];
        i1 = prj.Link[j as usize].node1;
        if i1 == i2 {
            i2 = prj.Link[j as usize].node2;
        } else {
            isCycle = FALSE;
            break;
        }
        i += 1;
    }

    // --- print cycle to report file
    if isCycle != 0 {
        kount = 0;
        i = 0;
        while i <= prj.toposort.LoopLinksLast {
            if kount % 5 == 0 {
                cprintf!(prj.Frpt, b"\n");
            }
            kount += 1;
            let lnk = prj.toposort.LoopLinks[i as usize] as usize;
            cprintf!(prj.Frpt, b"  %s", prj.Link[lnk].ID);
            if i < prj.toposort.LoopLinksLast {
                cprintf!(prj.Frpt, b"  -->");
            }
            i += 1;
        }
    }
}

//=============================================================================

fn traceLoop(prj: &mut Project, i1: i32, i2: i32, k1: i32) -> i32 {
//
//  Input:   i1 = index of current node reached while tracing a loop
//           i2 = index of final node on the loop
//           k1 = index of link which extends loop to node i1
//  Output:  returns TRUE if loop can be extended through node i1
//  Purpose: tries to extend closed loop through current node.
//
    let mut j: i32;
    let mut k: i32;
    let mut m: i32;

    // --- if current node equals final node then return with loop found
    if i1 == i2 {
        return TRUE;
    }

    // --- examine each link connected to current node
    m = prj.toposort.StartPos[i1 as usize];
    while m < prj.toposort.StartPos[i1 as usize] + prj.Node[i1 as usize].degree {
        // --- ignore link if it comes from predecessor node or if
        //     it is not on the spanning tree
        k = prj.toposort.AdjList[m as usize];
        if k == k1 || prj.toposort.InTree[k as usize] != 1 {
            m += 1;
            continue;
        }

        // --- identify other node that link is connected to
        if prj.Link[k as usize].node1 == i1 {
            j = prj.Link[k as usize].node2;
        } else {
            j = prj.Link[k as usize].node1;
        }

        // --- try to continue tracing the loop from this node;
        //     if successful, then add link to loop and return
        if traceLoop(prj, j, i2, k) != 0 {
            prj.toposort.LoopLinksLast += 1;
            let last = prj.toposort.LoopLinksLast as usize;
            prj.toposort.LoopLinks[last] = k;
            return TRUE;
        }
        m += 1;
    }

    // --- return false if loop cannot be continued from current node
    FALSE
}

//=============================================================================

fn checkDummyLinks(prj: &mut Project) {
//
//  Input:   none
//  Output:  none
//  Purpose: checks for nodes that have both incoming and outgoing dummy links.
//
    let mut j: i32;
    let mut marked: Vec<i32>;

    // --- create an array that marks nodes
    //     (calloc initializes the array to 0)
    marked = vec![0; prj.Nobjects[NODE as usize] as usize];

    // --- mark nodes that whose incoming links are all
    //     either dummy links or ideal pumps
    for i in 0..prj.Nobjects[LINK as usize] {
        let iu = i as usize;
        j = prj.Link[iu].node2;
        if prj.Link[iu].direction < 0 {
            j = prj.Link[iu].node1;
        }
        if (prj.Link[iu].type_ == CONDUIT && prj.Link[iu].xsect.type_ == DUMMY)
            || (prj.Link[iu].type_ == PUMP
                && prj.Pump[prj.Link[iu].subIndex as usize].type_ == IDEAL_PUMP)
        {
            if marked[j as usize] == 0 {
                marked[j as usize] = 1;
            }
        } else {
            marked[j as usize] = -1;
        }
    }

    // --- find marked nodes with outgoing dummy links or ideal pumps
    for i in 0..prj.Nobjects[LINK as usize] {
        let iu = i as usize;
        if (prj.Link[iu].type_ == CONDUIT && prj.Link[iu].xsect.type_ == DUMMY)
            || (prj.Link[iu].type_ == PUMP
                && prj.Pump[prj.Link[iu].subIndex as usize].type_ == IDEAL_PUMP)
        {
            j = prj.Link[iu].node1;
            if marked[j as usize] > 0 {
                let id = prj.Node[j as usize].ID.clone();
                report_writeErrorMsg(prj, ERR_DUMMY_LINK, &id);
            }
        }
    }
    drop(marked);
}

//=============================================================================
