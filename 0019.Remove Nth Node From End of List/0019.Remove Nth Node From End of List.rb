
def remove_nth_from_end(head, n)
    dummy = ListNode.new(0, head)
    fast = slow = dummy
    while n > 0
        fast = fast.next
        n -= 1
    end
    while fast.next
        slow = slow.next
        fast = fast.next
    end
    slow.next = slow.next.next
    return dummy.next
end
