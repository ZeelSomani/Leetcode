
function mergeKLists(lists: Array<ListNode | null>): ListNode | null {
    const pq = new MinPriorityQueue({ priority: (node: ListNode) => node.val });
    lists.filter(head => head).forEach(head => pq.enqueue(head));
    const dummy: ListNode = new ListNode();
    let cur: ListNode = dummy;
    while (!pq.isEmpty()) {
        const node = pq.dequeue().element;
        cur.next = node;
        cur = cur.next;
        if (node.next) {
            pq.enqueue(node.next);
        }
    }
    return dummy.next;
}
