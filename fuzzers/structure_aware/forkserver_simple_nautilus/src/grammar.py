ctx.rule("START", "{PROGRAM}")

ctx.rule("PROGRAM", "{INSTR}")
ctx.rule("PROGRAM", "{INSTR}{PROGRAM}")

ctx.rule("INSTR", "{OPCODE}")
ctx.rule("INSTR", "{PUSH1}")
ctx.rule("INSTR", "{PUSH2}")
ctx.rule("INSTR", "{PUSH3}")
ctx.rule("INSTR", "{PUSH4}")
ctx.rule("INSTR", "{PUSH5}")
ctx.rule("INSTR", "{PUSH6}")
ctx.rule("INSTR", "{PUSH7}")
ctx.rule("INSTR", "{PUSH8}")
ctx.rule("INSTR", "{PUSH9}")
ctx.rule("INSTR", "{PUSH10}")
ctx.rule("INSTR", "{PUSH11}")
ctx.rule("INSTR", "{PUSH12}")
ctx.rule("INSTR", "{PUSH13}")
ctx.rule("INSTR", "{PUSH14}")
ctx.rule("INSTR", "{PUSH15}")
ctx.rule("INSTR", "{PUSH16}")
ctx.rule("INSTR", "{PUSH17}")
ctx.rule("INSTR", "{PUSH18}")
ctx.rule("INSTR", "{PUSH19}")
ctx.rule("INSTR", "{PUSH20}")
ctx.rule("INSTR", "{PUSH21}")
ctx.rule("INSTR", "{PUSH22}")
ctx.rule("INSTR", "{PUSH23}")
ctx.rule("INSTR", "{PUSH24}")
ctx.rule("INSTR", "{PUSH25}")
ctx.rule("INSTR", "{PUSH26}")
ctx.rule("INSTR", "{PUSH27}")
ctx.rule("INSTR", "{PUSH28}")
ctx.rule("INSTR", "{PUSH29}")
ctx.rule("INSTR", "{PUSH30}")
ctx.rule("INSTR", "{PUSH31}")
ctx.rule("INSTR", "{PUSH32}")

ctx.rule("HEX", "0")
ctx.rule("HEX", "1")
ctx.rule("HEX", "2")
ctx.rule("HEX", "3")
ctx.rule("HEX", "4")
ctx.rule("HEX", "5")
ctx.rule("HEX", "6")
ctx.rule("HEX", "7")
ctx.rule("HEX", "8")
ctx.rule("HEX", "9")
ctx.rule("HEX", "a")
ctx.rule("HEX", "b")
ctx.rule("HEX", "c")
ctx.rule("HEX", "d")
ctx.rule("HEX", "e")
ctx.rule("HEX", "f")

ctx.rule("BYTE", "{HEX}{HEX}")

ctx.rule("OPCODE", "00")  # STOP
ctx.rule("OPCODE", "01")  # ADD
ctx.rule("OPCODE", "02")  # MUL
ctx.rule("OPCODE", "03")  # SUB
ctx.rule("OPCODE", "04")  # DIV
ctx.rule("OPCODE", "05")  # SDIV
ctx.rule("OPCODE", "06")  # MOD
ctx.rule("OPCODE", "07")  # SMOD
ctx.rule("OPCODE", "08")  # ADDMOD
ctx.rule("OPCODE", "09")  # MULMOD
ctx.rule("OPCODE", "0a")  # EXP
ctx.rule("OPCODE", "0b")  # SIGNEXTEND

ctx.rule("OPCODE", "10")  # LT
ctx.rule("OPCODE", "11")  # GT
ctx.rule("OPCODE", "12")  # SLT
ctx.rule("OPCODE", "13")  # SGT
ctx.rule("OPCODE", "14")  # EQ
ctx.rule("OPCODE", "15")  # ISZERO
ctx.rule("OPCODE", "16")  # AND
ctx.rule("OPCODE", "17")  # OR
ctx.rule("OPCODE", "18")  # XOR
ctx.rule("OPCODE", "19")  # NOT
ctx.rule("OPCODE", "1a")  # BYTE
ctx.rule("OPCODE", "1b")  # SHL
ctx.rule("OPCODE", "1c")  # SHR
ctx.rule("OPCODE", "1d")  # SAR

ctx.rule("OPCODE", "20")  # KECCAK256

ctx.rule("OPCODE", "30")  # ADDRESS
ctx.rule("OPCODE", "31")  # BALANCE
ctx.rule("OPCODE", "32")  # ORIGIN
ctx.rule("OPCODE", "33")  # CALLER
ctx.rule("OPCODE", "34")  # CALLVALUE
ctx.rule("OPCODE", "35")  # CALLDATALOAD
ctx.rule("OPCODE", "36")  # CALLDATASIZE
ctx.rule("OPCODE", "37")  # CALLDATACOPY
ctx.rule("OPCODE", "38")  # CODESIZE
ctx.rule("OPCODE", "39")  # CODECOPY
ctx.rule("OPCODE", "3a")  # GASPRICE
ctx.rule("OPCODE", "3b")  # EXTCODESIZE
ctx.rule("OPCODE", "3c")  # EXTCODECOPY
ctx.rule("OPCODE", "3d")  # RETURNDATASIZE
ctx.rule("OPCODE", "3e")  # RETURNDATACOPY
ctx.rule("OPCODE", "3f")  # EXTCODEHASH

ctx.rule("OPCODE", "40")  # BLOCKHASH
ctx.rule("OPCODE", "41")  # COINBASE
ctx.rule("OPCODE", "42")  # TIMESTAMP
ctx.rule("OPCODE", "43")  # NUMBER
ctx.rule("OPCODE", "44")  # PREVRANDAO
ctx.rule("OPCODE", "45")  # GASLIMIT
ctx.rule("OPCODE", "46")  # CHAINID
ctx.rule("OPCODE", "47")  # SELFBALANCE
ctx.rule("OPCODE", "48")  # BASEFEE
ctx.rule("OPCODE", "49")  # BLOBHASH
ctx.rule("OPCODE", "4a")  # BLOBBASEFEE

ctx.rule("OPCODE", "50")  # POP
ctx.rule("OPCODE", "51")  # MLOAD
ctx.rule("OPCODE", "52")  # MSTORE
ctx.rule("OPCODE", "53")  # MSTORE8
ctx.rule("OPCODE", "54")  # SLOAD
ctx.rule("OPCODE", "55")  # SSTORE
ctx.rule("OPCODE", "56")  # JUMP
ctx.rule("OPCODE", "57")  # JUMPI
ctx.rule("OPCODE", "58")  # PC
ctx.rule("OPCODE", "59")  # MSIZE
ctx.rule("OPCODE", "5a")  # GAS
ctx.rule("OPCODE", "5b")  # JUMPDEST
ctx.rule("OPCODE", "5c")  # TLOAD
ctx.rule("OPCODE", "5d")  # TSTORE
ctx.rule("OPCODE", "5e")  # MCOPY
ctx.rule("OPCODE", "5f")  # PUSH0

ctx.rule("OPCODE", "80")  # DUP1
ctx.rule("OPCODE", "81")  # DUP2
ctx.rule("OPCODE", "82")  # DUP3
ctx.rule("OPCODE", "83")  # DUP4
ctx.rule("OPCODE", "84")  # DUP5
ctx.rule("OPCODE", "85")  # DUP6
ctx.rule("OPCODE", "86")  # DUP7
ctx.rule("OPCODE", "87")  # DUP8
ctx.rule("OPCODE", "88")  # DUP9
ctx.rule("OPCODE", "89")  # DUP10
ctx.rule("OPCODE", "8a")  # DUP11
ctx.rule("OPCODE", "8b")  # DUP12
ctx.rule("OPCODE", "8c")  # DUP13
ctx.rule("OPCODE", "8d")  # DUP14
ctx.rule("OPCODE", "8e")  # DUP15
ctx.rule("OPCODE", "8f")  # DUP16

ctx.rule("OPCODE", "90")  # SWAP1
ctx.rule("OPCODE", "91")  # SWAP2
ctx.rule("OPCODE", "92")  # SWAP3
ctx.rule("OPCODE", "93")  # SWAP4
ctx.rule("OPCODE", "94")  # SWAP5
ctx.rule("OPCODE", "95")  # SWAP6
ctx.rule("OPCODE", "96")  # SWAP7
ctx.rule("OPCODE", "97")  # SWAP8
ctx.rule("OPCODE", "98")  # SWAP9
ctx.rule("OPCODE", "99")  # SWAP10
ctx.rule("OPCODE", "9a")  # SWAP11
ctx.rule("OPCODE", "9b")  # SWAP12
ctx.rule("OPCODE", "9c")  # SWAP13
ctx.rule("OPCODE", "9d")  # SWAP14
ctx.rule("OPCODE", "9e")  # SWAP15
ctx.rule("OPCODE", "9f")  # SWAP16

ctx.rule("OPCODE", "a0")  # LOG0
ctx.rule("OPCODE", "a1")  # LOG1
ctx.rule("OPCODE", "a2")  # LOG2
ctx.rule("OPCODE", "a3")  # LOG3
ctx.rule("OPCODE", "a4")  # LOG4

ctx.rule("OPCODE", "f0")  # CREATE
ctx.rule("OPCODE", "f1")  # CALL
ctx.rule("OPCODE", "f2")  # CALLCODE
ctx.rule("OPCODE", "f3")  # RETURN
ctx.rule("OPCODE", "f4")  # DELEGATECALL
ctx.rule("OPCODE", "f5")  # CREATE2
ctx.rule("OPCODE", "fa")  # STATICCALL
ctx.rule("OPCODE", "fd")  # REVERT
ctx.rule("OPCODE", "fe")  # INVALID
ctx.rule("OPCODE", "ff")  # SELFDESTRUCT

ctx.rule("PUSH1", "60{BYTE}")
ctx.rule("PUSH2", "61{BYTE}{BYTE}")
ctx.rule("PUSH3", "62{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH4", "63{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH5", "64{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH6", "65{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH7", "66{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH8", "67{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH9", "68{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH10", "69{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH11", "6a{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH12", "6b{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH13", "6c{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH14", "6d{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH15", "6e{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH16", "6f{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH17", "70{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH18", "71{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH19", "72{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH20", "73{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH21", "74{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH22", "75{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH23", "76{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH24", "77{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH25", "78{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH26", "79{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH27", "7a{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH28", "7b{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH29", "7c{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH30", "7d{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH31", "7e{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
ctx.rule("PUSH32", "7f{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}{BYTE}")
