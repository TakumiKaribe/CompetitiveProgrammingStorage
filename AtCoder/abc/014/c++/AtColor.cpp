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

#define MOD 1000000007

using namespace std;

typedef long long ll;

typedef vector<ll> vl;
typedef vector<int> vi;
typedef vector<vl> vvl;
typedef vector<vi> vvi;

typedef pair<ll, ll> pll;
typedef pair<int, int> pi;

typedef vector<pll> vpl;
typedef vector<pi> vpi;

typedef queue<ll> ql;
typedef queue<int> qi;

template <class T>
void printv(vector<T> vc) {
    for (auto e: vc) cout << e << " ";
    cout << endl;
}

template <class T>
void printvv(vector<T> vvc) {
    for (auto vc: vvc) {
        for (auto e: vc) cout << e << " ";
        cout << endl;
    }
}

template <class T>
void printvp(vector<pair<T, T> > vp) {
    for (auto pT: vp) cout << pT.first << " " << pT.second << endl;
}

int main() {
    ll n;
    cin >> n;
    vl vec;
    vec.resize(1000001);
    for (int i = 0; i < n; ++i) {
        ll a, b;
        cin >> a >> b;
        ++vec[a];
        --vec[b+1];
    }

    ll ans = 0;
    ll c = 0;
    for (int i = 0; i < 1000001; ++i) {
        c += vec[i];
        ans = max(ans, c);
    }

    cout << ans << endl;
    return 0;
}
