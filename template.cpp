#include <iostream>
#include <vector>
#include <set>
#include <string>
#include <algorithm>
#include <queue>
#include <utility>
#include <climits>
#include <bitset>
#include <cmath>
#include <map>
#include <tuple>
#include <complex>

#define MOD 1000000007

using namespace std;

typedef long long ll;

typedef vector<ll> vl;
typedef vector<int> vi;
typedef vector<vl> vvl;
typedef vector<vi> vvi;
typedef vector<string> vs;

typedef pair<ll, ll> pll;
typedef pair<int, int> pi;

typedef vector<pll> vpl;
typedef vector<pi> vpi;
typedef vector<vs> vvs;

typedef vector<pll> vvpl;
typedef vector<vpi> vvpi;

typedef queue<ll> ql;
typedef queue<int> qi;
typedef queue<pll> qpl;
typedef queue<pi> qpi;

template <typename T>
void printv(const vector<T>& v) {
    for (const auto& e : v) cout << e << " ";
    cout << endl;
}

template <typename T>
void printvv(const vector<T>& vv) {
    for (const auto& v : vv) {
        for (const auto& e : v) cout << e << " ";
        cout << endl;
    }
}

template <typename T, typename U>
void printp(const pair<T, U>& p) {
    cout << p.first << " " << p.second << endl;
}

template <typename T>
void printvp(const vector<pair<T, T>>& vp) {
    for (const auto& p : vp) cout << p.first << " " << p.second << endl;
}

template <typename T>
void printvvp(const vector<vector<pair<T, T>>>& vvp) {
    for (const auto& vp : vvp) for (const auto& p : vp) cout << p.first << " " << p.second << endl;
}

int main()
{
    return 0;
}
