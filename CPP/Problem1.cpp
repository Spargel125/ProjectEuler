#include <bits/stdc++.h>
#define rep(i,n) for(int i=0;i<(n);++i)
using namespace std;
using P = pair<int,int>;

int main(void){
    int num = 1000;
    int sum = 0;
    for(int i=1;i<num;i++){
        if(i%3==0 || i%5==0){
            sum += i;
        }
    }
    cout << sum << endl;
    return 0;
}