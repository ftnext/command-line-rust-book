def plus_one_ng(numbers: list[int]) -> list[int]:
    return [n + 1 for n in numbers]


def plus_one_ok(numbers: Iterable[int]) -> list[int]:
    return [n + 1 for n in numbers]
