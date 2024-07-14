class Solution(object):
    def countOfAtoms(self, formula):
        """
        :type formula: str
        :rtype: str
        """
        count = self.dfs(formula)
        res = ""
        for atom, num in sorted(count.items()):
            if num == 1:
                res += atom
            else:
                res += atom + str(num)
        return res
        
    def dfs(self, formula):
        count = collections.Counter()
        if not formula: return count
        i = 0
        while i < len(formula):
            if formula[i].isalpha(): # 首字母是英文字符
                atom = formula[i]
                atomNum = 0
                # 找到这个元素所有字符
                i += 1
                while i < len(formula) and formula[i].isalpha() and formula[i].islower():
                    atom += formula[i]
                    i += 1
                while i < len(formula) and formula[i].isdigit(): # 后面是否有数字
                    atomNum = 10 * atomNum + int(formula[i])
                    i += 1
                count[atom] += 1 if atomNum == 0 else atomNum　＃　使用加号
            elif formula[i] == "(": # 括号匹配
                left = i # 左括号位置
                parent = 1 #　统计括号个数
                while i < len(formula) and parent != 0:
                    i += 1
                    if formula[i] == "(":
                        parent += 1
                    elif formula[i] == ")":
                        parent -= 1
                right = i
                atomNum = 0
                i += 1
                while i < len(formula) and formula[i].isdigit(): # 后面是否有数字
                    atomNum = 10 * atomNum + int(formula[i])
                    i += 1
                innerCount = self.dfs(formula[left + 1 : right])
                for c, n in innerCount.items():
                    count[c] += n * atomNum
        count += self.dfs(formula[i + 1 :])
        return count
