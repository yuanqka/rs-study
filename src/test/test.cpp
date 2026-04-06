#include <iostream>
using namespace std;

int main(){
    int a = 1;
    test(a);
    return 0;
}
//initializing argument 1 of ‘void test(int&)’

class Solution {
public:
    long long minimumCost(vector<int>& nums, int k, int dist) {
        int len = nums.size();
        k -= 1;
        long long ans = nums[0];
        priority_queue<int> g_pq;
        priority_queue<int, vector<int>, greater<int>> l_pq;
        unordered_map<int, int> map;
        for (int i = 1; i <= dist + 1; i += 1) {
            if (g_pq.size() < k) {
                g_pq.push(nums[i]);
                ans += nums[i];
            } else {
                if (g_pq.top() > nums[i]){
                    int temp = g_pq.top();
                    g_pq.pop();
                    g_pq.push(nums[i]);
                    l_pq.push(temp);
                    ans += nums[i] - temp;
                } else {
                    l_pq.push(nums[i]);
                }
            }
        }
        long long sum = ans;
        auto clean = [&] {
            while (map.find(g_pq.top()) != map.end()) {
                map[g_pq.top()] -= 1;
                if (map[g_pq.top()] == 0) map.erase(g_pq.top());
                g_pq.pop();
            }
            while (!l_pq.empty() && map.find(l_pq.top()) != map.end()) {
                map[l_pq.top()] -= 1;
                if (map[l_pq.top()] == 0) map.erase(l_pq.top());
                l_pq.pop();
            }
        };
        for (int i = 2; i < len - dist; i += 1) {
            if (i == 4) {
                return g_pq.size();
            }
            clean();
            int out = nums[i - 1], in = nums[i + dist];
            map[out] += 1;
            if (g_pq.top() > in) {
                int temp = g_pq.top();
                g_pq.pop();
                g_pq.push(in);
                l_pq.push(temp);
                sum += in - temp;
            } else {
                l_pq.push(in);
            }
            if (i == 4) {
                return g_pq.size();
            }
            clean();
            //if (g_pq.size() < 2) return i;
            if (g_pq.top() >= out) {
                sum -= out;//淘汰的元素在大顶堆里, 不应该计入sum;
                int temp = l_pq.top();
                l_pq.pop();
                g_pq.push(temp);
                sum += temp;//此时sum是i~i+dist-1间的最小的k(新k)个数, 不能直接更新
            }
            if (i == 3) {
                return g_pq.top();
            }
            ans = min(ans, sum);
        }
        return ans;
    }
};