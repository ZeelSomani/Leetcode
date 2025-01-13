class Solution {
    function search($nums, $target) {
        $foundKey = -1;
        foreach ($nums as $key => $value) {
            if ($value === $target) {
                $foundKey = $key;
            }
        }
        return $foundKey;
    }
}
