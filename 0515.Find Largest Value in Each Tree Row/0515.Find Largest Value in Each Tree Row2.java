
class Solution {
    private List<Integer> ans = new ArrayList<>();

    public List<Integer> largestValues(TreeNode root) {
        dfs(root, 0);
        return ans;
    }

    private void dfs(TreeNode root, int curr) {
        if (root == null) {
            return;
        }
        if (curr == ans.size()) {
            ans.add(root.val);
        } else {
            ans.set(curr, Math.max(ans.get(curr), root.val));
        }
        dfs(root.left, curr + 1);
        dfs(root.right, curr + 1);
    }
}
