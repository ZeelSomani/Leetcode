

function largestValues(root: TreeNode | null): number[] {
    const ans: number[] = [];
    if (!root) {
        return ans;
    }
    const q: TreeNode[] = [root];
    while (q.length) {
        const nq: TreeNode[] = [];
        let x = -Infinity;
        for (const { val, left, right } of q) {
            x = Math.max(x, val);
            if (left) {
                nq.push(left);
            }
            if (right) {
                nq.push(right);
            }
        }
        ans.push(x);
        q.length = 0;
        q.push(...nq);
    }
    return ans;
}
