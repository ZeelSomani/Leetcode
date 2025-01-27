
var mergeKLists = function (lists) {
    const pq = new MinPriorityQueue({ priority: node => node.val });
    lists.filter(head => head).forEach(head => pq.enqueue(head));
    const dummy = new ListNode();
    let cur = dummy;
    while (!pq.isEmpty()) {
        const node = pq.dequeue().element;
        cur.next = node;
        cur = cur.next;
        if (node.next) {
            pq.enqueue(node.next);
        }
    }
    return dummy.next;
};
