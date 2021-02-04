#include <bits/stdc++.h>
#define rep(i,n) for(int i=0;i<(n);++i)
using namespace std;
using P = pair<int,int>;

int main(void){
    long long int num = 600851475143;
    long long int i = 2;
    while(i<num){
        while(num%i==0){
            num = num/i;
        }
        i++;
        // cout << i<< endl;
    }
    cout << num << endl;
    return 0;
}