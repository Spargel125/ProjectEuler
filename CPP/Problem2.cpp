#include <bits/stdc++.h>
#define rep(i,n) for(int i=0;i<(n);++i)
using namespace std;
using P = pair<int,int>;

int calc_fib(int a,int b){
    return a+b;
}

int main(void){
    int first =1;
    int second = 2;
    int temp = 0;
    int sum = 2;
    while(temp<4000000){
        temp=calc_fib(first,second);
        first = second;
        second = temp;
        if(temp%2 == 0){
            sum += temp;
        }
    }
    cout << sum << endl;
    return 0;
}