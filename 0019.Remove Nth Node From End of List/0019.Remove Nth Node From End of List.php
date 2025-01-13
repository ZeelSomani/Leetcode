
class Solution {
   
    function removeNthFromEnd($head, $n) {
        $dummy = new ListNode(0, $head);
        $fast = $slow = $dummy;
        for ($i = 0; $i < $n; $i++) {
            $fast = $fast->next;
        }
        while ($fast->next !== null) {
            $fast = $fast->next;
            $slow = $slow->next;
        }
        $slow->next = $slow->next->next;
        return $dummy->next;
    }
}
