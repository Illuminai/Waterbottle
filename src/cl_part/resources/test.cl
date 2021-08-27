
__kernel void test(__global uchar* ret) {
    printf("Global work item %li with local id %li calls in\n", get_global_id(0), get_local_id(0));
}

struct pair_t {
    int begin;
    int end;
    int type;
};

__kernel void find_pairs(
                    __global uchar* document,
                    __global int* begin,
                    __global int* end,
                    __global uchar* type,
                    __global int* amount) {
    printf("Global work item %li with local id %li calls in\n", get_global_id(0), get_local_id(0));
    *amount = 1024;
}
