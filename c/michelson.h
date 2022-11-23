typedef long int Mutez; // signed 64-bit
typedef long long int Int;
typedef unsigned long long int Nat;
typedef int Contract;
typedef int Operation;
typedef char* Address;
#define DUMMY_AMOUNT 0
#define DUMMY_BALANCE 0
#define DUMMY_CONTRACT 0
#define DUMMY_OPERATION 0;
#define DUMMY_ADDRESS "KT1xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
#define DUMMY_CHAIN_ID 0
#define DUMMY_NOW 0
#define DUMMY_SELF_ADDRESS 0
#define DUMMY_SENDER 0
#define DUMMY_SOURCE 0
#define DUMMY_LEVEL 0
#define DUMMY_TOTAL_VOTING_POWER 0

struct Parameter;
struct Storage;

struct Pair; // pair ( list operation, storage)

Mutez get_amount() {
    return DUMMY_AMOUNT;
}

Mutez get_balance() {
    return DUMMY_BALANCE;
}

Nat get_total_voting_power() {
    return DUMMY_TOTAL_VOTING_POWER;
}

Address get_self_address() {
    return DUMMY_ADDRESS;
}

Address get_sender() {
    return DUMMY_ADDRESS;
}

Address get_source() {
    return DUMMY_ADDRESS;
}

Nat get_level() {
    return DUMMY_LEVEL;
}

Contract get_contract(Address addr) {
    return DUMMY_CONTRACT;
}

Operation transfer_tokens(struct Parameter param, Mutez tokens, Contract contract) {
    return DUMMY_OPERATION;
}
