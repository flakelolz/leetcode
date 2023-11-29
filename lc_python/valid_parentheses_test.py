class Solution:
    def isValid(self, s: str) -> bool:
        stack = []
        closed = {")": "(", "}": "{", "]": "["}
        for c in s:
            if c in closed:
                if stack and stack[-1] == closed[c]:
                    stack.pop()
                else:
                    return False
            else:
                stack.append(c)

        return True if not stack else False


solution = Solution()


def test_isValid_1():
    s = "()"
    assert solution.isValid(s) is True


def test_isValid_2():
    s = "()[]{}"
    assert solution.isValid(s) is True


def test_isValid_3():
    s = "(]"
    assert solution.isValid(s) is False

