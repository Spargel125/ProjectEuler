#include <bits/stdc++.h>
#define rep(i,n) for(int i=0;i<(n);++i)
using namespace std;
using P = pair<int,int>;

int main(void){
    string str;
    string str_rev;
    long long int num = 0;
    long long int maxnum = 0;
    for(int i=100;i<1000;i++){
        for(int j=100;j<1000;j++){
            num = i*j;
            str = to_string(num);
            str_rev = to_string(num);
            reverse(str_rev.begin(),str_rev.end());
            if(str == str_rev){
                if(maxnum < num){
                    maxnum = num;
                }
            }
        }
    }
    cout << maxnum << endl;
    return 0;
}