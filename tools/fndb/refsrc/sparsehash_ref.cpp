// Reference instantiations of sparsehash for fndb library identification (features only).
#include <sparsehash/dense_hash_map>
#include <sparsehash/dense_hash_set>
#include <sparsehash/sparse_hash_map>
#include <sparsehash/sparse_hash_set>
#include <string>
#include <functional>
#define X __declspec(dllexport)
using google::dense_hash_map; using google::dense_hash_set; using google::sparse_hash_map; using google::sparse_hash_set;
struct P { float a; int b; };
X int dh_si(dense_hash_map<std::string, int>& m, const std::string& k) { return m[k]; }
X bool dh_si_find(const dense_hash_map<std::string, int>& m, const std::string& k) { return m.find(k) != m.end(); }
X void dh_si_erase(dense_hash_map<std::string, int>& m, const std::string& k) { m.erase(k); }
X void* dh_up(dense_hash_map<unsigned long long, void*>& m, unsigned long long k) { return m[k]; }
X bool dh_up_find(const dense_hash_map<unsigned long long, void*>& m, unsigned long long k) { return m.find(k) != m.end(); }
X void dh_up_erase(dense_hash_map<unsigned long long, void*>& m, unsigned long long k) { m.erase(k); }
X void dh_up_ins(dense_hash_map<unsigned long long, void*>& m, unsigned long long k, void* v) { m.insert(std::make_pair(k, v)); }
X int dh_ii(dense_hash_map<int, int>& m, int k) { return m[k]; }
X void dh_ii_resize(dense_hash_map<int, int>& m, size_t n) { m.resize(n); }
X P dh_pP(dense_hash_map<void*, P>& m, void* k) { return m[k]; }
X void dh_init(dense_hash_map<unsigned long long, void*>& m) { m.set_empty_key(~0ull); m.set_deleted_key(~0ull - 1); }
X void dh_clear(dense_hash_map<std::string, int>& m) { m.clear(); }
X dense_hash_map<unsigned long long, void*> dh_copy(const dense_hash_map<unsigned long long, void*>& m) { return m; }
X bool ds_i(dense_hash_set<int>& s, int k) { return s.insert(k).second; }
X bool ds_p(dense_hash_set<void*>& s, void* k) { return s.insert(k).second; }
X bool ds_s(dense_hash_set<std::string>& s, const std::string& k) { return s.insert(k).second; }
X void ds_erase(dense_hash_set<void*>& s, void* k) { s.erase(k); }
X int sh_si(sparse_hash_map<std::string, int>& m, const std::string& k) { return m[k]; }
X void* sh_up(sparse_hash_map<unsigned long long, void*>& m, unsigned long long k) { return m[k]; }
X bool sh_up_find(const sparse_hash_map<unsigned long long, void*>& m, unsigned long long k) { return m.find(k) != m.end(); }
X void sh_up_erase(sparse_hash_map<unsigned long long, void*>& m, unsigned long long k) { m.erase(k); }
X bool ss_p(sparse_hash_set<void*>& s, void* k) { return s.insert(k).second; }
X sparse_hash_map<unsigned long long, void*> sh_copy(const sparse_hash_map<unsigned long long, void*>& m) { return m; }
