#include<iostream>
#include<vector>
# define INT_MIN -9999;
using namespace std;

int Kadane_max_subarray_sum_(const vector<int>& arr){
    int _max = INT_MIN;
    int max_pos = 0; 

    for(int num: arr)
    {
        max_pos += num;
        if(_max<max_pos)    _max = max_pos;
        if(max_pos<0)   max_pos=0;
    }
    return _max;

}


