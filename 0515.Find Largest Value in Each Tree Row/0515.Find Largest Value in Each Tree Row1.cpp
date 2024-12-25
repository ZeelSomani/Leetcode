
class Solution {
public:
    vector<int> largestValues(TreeNode* root) {
        vector<int> ans;
        if (!root) {
            return ans;
        }
        queue<TreeNode*> q{{root}};
        while (q.size()) {
            int x = INT_MIN;
            for (int i = q.size(); i; --i) {
                TreeNode* node = q.front();
                q.pop();
                x = max(x, node->val);
                if (node->left) {
                    q.push(node->left);
                }
                if (node->right) {
                    q.push(node->right);
                }
            }
            ans.push_back(x);
        }
        return ans;
    }
};
