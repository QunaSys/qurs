#include "operator.hpp"
#include <cstdio>
#include <stdlib.h>
#include "test.cpp"
typedef double _Complex CTYPE;
Operator::Operator()
{
    std::printf("Hello from C++, operator initialized");
}

double *Operator::X(double *arr)
{
    double *newArr = (double *)malloc(2 * sizeof(double));
    newArr[0] = arr[0];
    newArr[1] = arr[1];
    return arr;
}
