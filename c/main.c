//sample code
#include "michelson.h"

struct Parameter { };
struct Storage {
    Mutez amount;
    Mutez balance;
    Nat total_voting_power;
};

struct Pair {
    Operation ops[3];
    struct Storage storage;
};

struct Pair smart_contract(struct Parameter param, struct Storage storage) {
    struct Pair p;
    Address addr = "tz1ddb9NMYHZi5UzPdzTZMYQQZoMub195zgv";
    Contract contract = get_contract(addr);
    struct Parameter param2;
    Operation op = transfer_tokens(param2, 100, contract);
    p.ops[1] = op;
    return p;
};
