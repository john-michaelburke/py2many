from typing import List


def do_pass():
    pass


def inline_pass(): pass


def indexing():
    sum = 0
    a: List[int] = []
    for i in range(10):
        a.append(i)
        sum += a[i]
    return sum


def infer_bool(code: int):
    return code in [1, 2, 4]


def show():
    # assign
    a1 = 10
    #b1 = b2 = 15
    #assert b2 == 15
    b9 = 2; b2 = 2;
    assert b2 == b9
    # annotated assign
    a2: float = 2.1
    print(a2)
    # for loop
    for i in range(0, 10):
        print(i)
    for i in range(0, 10, 2):
        print(i)
    # for i in range(1, 10):
    #     print(i)
    # unary op
    a3 = -a1
    # binary op
    a4 = a3 + a1
    print(a4)
    sum1 = indexing()
    print(sum1)
    # lists, sets and dict
    a5 = [1, 2, 3]
    print(len(a5))
    a9: List[str] = ["a", "b", "c", "d"]
    print(len(a9))
    a6 = {1, 2, 3, 4}
    print(len(a6))
    a7 = {"a": 1, "b": 2}
    print(len(a7))
    a8 = True
    # print()
    if a8:
        print("true")
    elif a4 > 0:
        print("never get here")
    if a1 == 11:
        print("false")
    else:
        print("true")
    if 1 != None:
        print("World is sane")
    # print(True)
    # if True: a1 += 1
    # if True: print("true")
    do_pass()
    inline_pass()
    s = '1\
    2'
    print(s)
    # assert 1 != 2 != 3


if __name__ == "__main__":
    show()
