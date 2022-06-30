class OutValue:
    result: str
    correct: int
    is_correct: bool


class Outputer:
    def output_table(self, values: [[OutValue]], durations: [int], runtime: int) -> str:
        print('Results')
        [[print(a.result) for a in b] for b in values]

        print('Durations')
        [print(a) for a in durations]

        print('Runtime: ' + str(runtime))
        return ""
