#include <stdio.h>
#include <stdlib.h>

// Function prototype declaration
int calculate_sum(int a, int b);

int main(void) {
    // 1. Memory Leak (Error/Warning)
    // The memory allocated here is never freed.
    int *ptr = (int *)malloc(sizeof(int) * 5); 
    if (ptr == NULL) {
        return 1;
    }
    
    // 2. Array Index Out of Bounds (Error)
    // 'ptr' has 5 elements (indices 0 to 4). Accessing index 5 is out of bounds.
    ptr[4] = 10; // This is a critical error

    // 3. Unused Variable (Warning/Style)
    int unused_value = 99; // 'unused_value' is assigned but never used.

    // 4. Division by Zero (Warning/Logic)
    int zero_divisor = 0;
    int result = 100 / zero_divisor; // Potential division by zero.

    // 5. Portability Issue (Style/Portability)
    // Using a non-standard length modifier (l) with an int (%d) in printf.
    printf("Result: %ld\n", result); 

    // Function call
    int total = calculate_sum(10, 20);

    // Missing 'free(ptr);' is the cause of the memory leak (Issue 1)

    return 0;
}

// 6. Unused Function Parameter (Warning/Style)
// 'a' is declared but not used in the function body.
int calculate_sum(int a, int b) {
    return b + 10;
}