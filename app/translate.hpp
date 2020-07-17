#ifndef CATSATMAT_TRANSLATE
#define  CATSATMAT_TRANSLATE

#include <vector>

using namespace std;

struct SpaceChar{
    int h, w;
    vector<vector<char>> val;
    SpaceChar(int h = 0, int w = 0):w(w),h(h),val(h, vector<char>(w, 0)){}
    SpaceChar(vector<vector<char>> src):w(src.size()),h(src[0].size()),val(src){}

    vector<char> &operator[](int i){
        return val[i];
    }
};

using SpaceLine = vector<SpaceChar>;

template<class T>
vector<vector<T>> transpose(vector<vector<T>> x){
    int h = x.size();
    int w = x[0].size();
    vector<vector<T>> res(w, vector<T>(h));
    for(int i = 0;i < h;i++){
        for(int j = 0;j < w;j++){
            res[j][i] = x[i][j];
        }
    }
    return res;
}

SpaceLine translate_one_line(vector<vector<char>> src){
    SpaceLine translated;
    int h = src.size();
    int w = src[0].size();
    vector<vector<char>> one_letter;
    for(int j = 0;j < w;j++){
        int one_cnt = 0;
        vector<char> column;
        for(int i = 0;i < h;i++){
            if(src[i][j])one_cnt++;
            column.push_back(src[i][j]);
        }
        if(one_cnt == 0){
            if(!one_letter.empty()){
                translated.push_back(SpaceChar(transpose(one_letter)));
                one_letter.clear();
            }
        }
        else{
            one_letter.push_back(column);
        }
    }
    if(!one_letter.empty())translated.push_back(SpaceChar(transpose(one_letter)));
    return translated;
}

vector<SpaceLine> translate(vector<vector<char>> src){
    int h = src.size();
    int w = src[0].size();
    vector<SpaceLine> translated;
    for(int i = 0;i < h;i++)src[i][0] = src[i][w-1] = 0;
    for(int j = 0;j < w;j++)src[0][j] = src[h-1][j] = 0;
    vector<vector<char>> src_line;
    for(int i = 0;i < h;i++){
        int one_cnt = 0;
        for(int j = 0;j < w;j++){
            if(src[i][j])one_cnt++;
        }
        if(one_cnt == 0){
            if(!src_line.empty()){
                translated.push_back(translate_one_line(src_line));
                src_line.clear();
            }
        }
        else{
            src_line.push_back(src[i]);
        }
    }
    if(!src_line.empty())translated.push_back(translate_one_line(src_line));
    return translated;
}

#endif
