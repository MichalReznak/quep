from typing import TypedDict


class MetaInfo(TypedDict):
    time: float


class QcProvider:
    def get_meta_info(self) -> MetaInfo:
        return {'time': 442}

    def auth(self):
        # Any authorization necessary
        pass

    def clear_circuits(self: 'QcProvider'):
        # Clear circuits
        pass

    def append_circuit(self: 'QcProvider', circuit: str, t: str, log: bool):
        # These are the circuits that will be executed
        pass

    def run_all(self: 'QcProvider') -> dict[str, int]:
        # Execute on provider and return the result
        # in a form of str
        return {'0100': 40002}
