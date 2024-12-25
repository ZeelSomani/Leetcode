
class Solution {
public:
    vector<int> ans;

    vector<int> largestValues(TreeNode* root) {
        dfs(root, 0);
        return ans;
    }

    void dfs(TreeNode* root, int curr) {
        if (!root) return;
        if (curr == ans.size())
            ans.push_back(root->val);
        else
            ans[curr] = max(ans[curr], root->val);
        dfs(root->left, curr + 1);
        dfs(root->right, curr + 1);
    }
};
