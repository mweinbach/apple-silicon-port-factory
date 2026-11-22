// Reference instantiations of MSVC STL templates for fndb library identification.
// Compiled with clang-cl against microsoft/STL headers (see tools/fndb/libcorpus.py refbuild);
// the resulting objects are only mined for features (constants, strings, tables, CFG, imports).
#include <algorithm>
#include <atomic>
#include <cstring>
#include <deque>
#include <functional>
#include <list>
#include <map>
#include <memory>
#include <mutex>
#include <set>
#include <sstream>
#include <string>
#include <string_view>
#include <unordered_map>
#include <unordered_set>
#include <vector>

#define X __declspec(dllexport)

struct P { float a; int b; };
struct Big { char d[40]; };

// --- hashing
X size_t h_str(const std::string& s) { return std::hash<std::string>{}(s); }
X size_t h_sv(std::string_view s) { return std::hash<std::string_view>{}(s); }
X size_t h_wstr(const std::wstring& s) { return std::hash<std::wstring>{}(s); }
X size_t h_u64(unsigned long long v) { return std::hash<unsigned long long>{}(v); }
X size_t h_ptr(void* v) { return std::hash<void*>{}(v); }
X size_t h_bytes(const void* p, size_t n) { return std::_Hash_array_representation(static_cast<const unsigned char*>(p), n); }

// --- string search / modify
X size_t s_ffo(const std::string& s, const char* c) { return s.find_first_of(c); }
X size_t s_flo(const std::string& s, const char* c) { return s.find_last_of(c); }
X size_t s_ffno(const std::string& s, const char* c) { return s.find_first_not_of(c); }
X size_t s_flno(const std::string& s, const char* c) { return s.find_last_not_of(c); }
X size_t s_find(const std::string& s, const char* c) { return s.find(c); }
X size_t s_rfind(const std::string& s, const char* c) { return s.rfind(c); }
X size_t s_findc(const std::string& s, char c) { return s.find(c); }
X size_t sv_ffo(std::string_view s, std::string_view c) { return s.find_first_of(c); }
X size_t sv_flo(std::string_view s, std::string_view c) { return s.find_last_of(c); }
X size_t sv_find(std::string_view s, std::string_view c) { return s.find(c); }
X size_t w_ffo(const std::wstring& s, const wchar_t* c) { return s.find_first_of(c); }
X void s_append(std::string& s, const char* c) { s.append(c); }
X void s_appendn(std::string& s, const char* c, size_t n) { s.append(c, n); }
X void s_push(std::string& s, char c) { s.push_back(c); }
X void s_insert(std::string& s, size_t at, const char* c) { s.insert(at, c); }
X void s_erase(std::string& s, size_t at, size_t n) { s.erase(at, n); }
X void s_replace(std::string& s, size_t at, size_t n, const char* c) { s.replace(at, n, c); }
X void s_resize(std::string& s, size_t n) { s.resize(n); }
X void s_reserve(std::string& s, size_t n) { s.reserve(n); }
X void s_assign(std::string& s, const char* c, size_t n) { s.assign(c, n); }
X std::string s_substr(const std::string& s, size_t a, size_t n) { return s.substr(a, n); }
X std::string s_concat(const std::string& a, const std::string& b) { return a + b; }
X int s_compare(const std::string& a, const std::string& b) { return a.compare(b); }
X std::string s_copy(const std::string& a) { return a; }
X void w_append(std::wstring& s, const wchar_t* c) { s.append(c); }
X std::string s_tostr(int v) { return std::to_string(v); }
X std::string s_tostr_d(double v) { return std::to_string(v); }
X std::string s_tostr_u64(unsigned long long v) { return std::to_string(v); }
X int s_stoi(const std::string& s) { return std::stoi(s); }
X double s_stod(const std::string& s) { return std::stod(s); }

// --- sort / algorithms
X void a_sort_i(std::vector<int>& v) { std::sort(v.begin(), v.end()); }
X void a_sort_u64(std::vector<unsigned long long>& v) { std::sort(v.begin(), v.end()); }
X void a_sort_f(std::vector<float>& v) { std::sort(v.begin(), v.end()); }
X void a_sort_p(std::vector<void*>& v) { std::sort(v.begin(), v.end()); }
X void a_sort_s(std::vector<std::string>& v) { std::sort(v.begin(), v.end()); }
X void a_sort_P(std::vector<P>& v) { std::sort(v.begin(), v.end(), [](const P& x, const P& y) { return x.a < y.a; }); }
X void a_sort_pp(std::vector<P*>& v) { std::sort(v.begin(), v.end(), [](const P* x, const P* y) { return x->b < y->b; }); }
X void a_ssort_i(std::vector<int>& v) { std::stable_sort(v.begin(), v.end()); }
X void a_ssort_P(std::vector<P>& v) { std::stable_sort(v.begin(), v.end(), [](const P& x, const P& y) { return x.b < y.b; }); }
X void a_nth(std::vector<float>& v, size_t n) { std::nth_element(v.begin(), v.begin() + n, v.end()); }
X void a_psort(std::vector<int>& v, size_t n) { std::partial_sort(v.begin(), v.begin() + n, v.end()); }
X void a_heap(std::vector<int>& v) { std::make_heap(v.begin(), v.end()); std::sort_heap(v.begin(), v.end()); }
X void a_push_heap(std::vector<P>& v, P p) { v.push_back(p); std::push_heap(v.begin(), v.end(), [](const P& x, const P& y) { return x.a < y.a; }); }
X int* a_lb(std::vector<int>& v, int k) { return &*std::lower_bound(v.begin(), v.end(), k); }
X int* a_ub(std::vector<int>& v, int k) { return &*std::upper_bound(v.begin(), v.end(), k); }
X void a_unique(std::vector<int>& v) { v.erase(std::unique(v.begin(), v.end()), v.end()); }
X void a_rev(std::vector<int>& v) { std::reverse(v.begin(), v.end()); }
X void a_remove(std::vector<int>& v, int k) { v.erase(std::remove(v.begin(), v.end(), k), v.end()); }
X int* a_find(std::vector<int>& v, int k) { return &*std::find(v.begin(), v.end(), k); }
X void a_rotate(std::vector<int>& v, size_t n) { std::rotate(v.begin(), v.begin() + n, v.end()); }

// --- vector growth
X void v_push_i(std::vector<int>& v, int x) { v.push_back(x); }
X void v_push_u64(std::vector<unsigned long long>& v, unsigned long long x) { v.push_back(x); }
X void v_push_p(std::vector<void*>& v, void* x) { v.push_back(x); }
X void v_push_P(std::vector<P>& v, const P& x) { v.push_back(x); }
X void v_push_B(std::vector<Big>& v, const Big& x) { v.push_back(x); }
X void v_push_s(std::vector<std::string>& v, const std::string& x) { v.push_back(x); }
X void v_emplace_s(std::vector<std::string>& v, const char* x) { v.emplace_back(x); }
X void v_resize_i(std::vector<int>& v, size_t n) { v.resize(n); }
X void v_resize_B(std::vector<Big>& v, size_t n) { v.resize(n); }
X void v_reserve_P(std::vector<P>& v, size_t n) { v.reserve(n); }
X void v_insert_i(std::vector<int>& v, size_t at, int x) { v.insert(v.begin() + at, x); }
X void v_insert_r(std::vector<int>& v, const int* a, const int* b) { v.insert(v.end(), a, b); }
X void v_erase_i(std::vector<int>& v, size_t at) { v.erase(v.begin() + at); }
X void v_assign_i(std::vector<int>& v, const int* a, const int* b) { v.assign(a, b); }
X std::vector<int> v_copy(const std::vector<int>& v) { return v; }
X std::vector<std::string> v_copy_s(const std::vector<std::string>& v) { return v; }
X void v_bool(std::vector<bool>& v, bool b) { v.push_back(b); }
X void v_shared(std::vector<std::shared_ptr<P>>& v, std::shared_ptr<P> p) { v.push_back(std::move(p)); }
X void v_unique(std::vector<std::unique_ptr<P>>& v) { v.push_back(std::make_unique<P>()); }

// --- unordered containers
X int um_si(std::unordered_map<std::string, int>& m, const std::string& k) { return m[k]; }
X bool um_si_find(const std::unordered_map<std::string, int>& m, const std::string& k) { return m.find(k) != m.end(); }
X void um_si_erase(std::unordered_map<std::string, int>& m, const std::string& k) { m.erase(k); }
X int um_ii(std::unordered_map<int, int>& m, int k) { return m[k]; }
X void* um_up(std::unordered_map<unsigned long long, void*>& m, unsigned long long k) { return m[k]; }
X bool um_up_find(const std::unordered_map<unsigned long long, void*>& m, unsigned long long k) { return m.count(k) != 0; }
X void um_up_erase(std::unordered_map<unsigned long long, void*>& m, unsigned long long k) { m.erase(k); }
X void um_pp(std::unordered_map<void*, P>& m, void* k, P p) { m.emplace(k, p); }
X void um_clear(std::unordered_map<std::string, int>& m) { m.clear(); }
X void um_rehash(std::unordered_map<int, int>& m, size_t n) { m.reserve(n); }
X bool us_i(std::unordered_set<int>& s, int k) { return s.insert(k).second; }
X bool us_s(std::unordered_set<std::string>& s, const std::string& k) { return s.insert(k).second; }
X bool us_p(std::unordered_set<void*>& s, void* k) { return s.insert(k).second; }
X void us_erase(std::unordered_set<void*>& s, void* k) { s.erase(k); }

// --- ordered containers
X int m_si(std::map<std::string, int>& m, const std::string& k) { return m[k]; }
X int m_ii(std::map<int, int>& m, int k) { return m[k]; }
X void m_erase(std::map<int, int>& m, int k) { m.erase(k); }
X bool m_find(const std::map<unsigned long long, P>& m, unsigned long long k) { return m.find(k) != m.end(); }
X bool s_i(std::set<int>& s, int k) { return s.insert(k).second; }
X bool s_s(std::set<std::string>& s, const std::string& k) { return s.insert(k).second; }
X void s_erase(std::set<int>& s, int k) { s.erase(k); }
X void mm_i(std::multimap<int, P>& m, int k, P p) { m.emplace(k, p); }

// --- list / deque
X void l_push(std::list<P>& l, P p) { l.push_back(p); }
X void l_sort(std::list<int>& l) { l.sort(); }
X void l_remove(std::list<int>& l, int k) { l.remove(k); }
X void d_push(std::deque<int>& d, int x) { d.push_back(x); }
X void d_pushf(std::deque<P>& d, P x) { d.push_front(x); }
X void d_pop(std::deque<int>& d) { d.pop_front(); }

// --- function / smart pointers
X void f_call(const std::function<void(int)>& f, int x) { f(x); }
X std::function<int(int)> f_make(int k) { return [k](int x) { return x + k; }; }
X std::function<void()> f_copy(const std::function<void()>& f) { return f; }
X std::shared_ptr<P> sp_make() { return std::make_shared<P>(); }
X std::shared_ptr<std::string> sp_make_s(const char* c) { return std::make_shared<std::string>(c); }
X std::shared_ptr<P> sp_new() { return std::shared_ptr<P>(new P()); }
X std::weak_ptr<P> sp_weak(const std::shared_ptr<P>& p) { return p; }
X std::shared_ptr<P> sp_lock(const std::weak_ptr<P>& p) { return p.lock(); }

// --- streams / misc
X std::string ss_fmt(int a, double b, const char* c) { std::ostringstream o; o << a << " " << b << c; return o.str(); }
X int ss_parse(const std::string& s) { std::istringstream i(s); int v = 0; i >> v; return v; }
X void mtx(std::mutex& m, int& v) { std::lock_guard<std::mutex> g(m); ++v; }
X void rmtx(std::recursive_mutex& m, int& v) { std::lock_guard<std::recursive_mutex> g(m); ++v; }
X int at_inc(std::atomic<int>& a) { return a.fetch_add(1); }
