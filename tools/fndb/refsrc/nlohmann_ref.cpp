// Reference instantiations of nlohmann::json (used by opentelemetry-cpp) for fndb (features only).
#include <nlohmann/json.hpp>
#define X __declspec(dllexport)
using nlohmann::json;
X json nj_parse(const std::string& s) { return json::parse(s); }
X std::string nj_dump(const json& j) { return j.dump(); }
X std::string nj_dump_i(const json& j) { return j.dump(2); }
X void nj_set(json& j, const std::string& k, int v) { j[k] = v; }
X void nj_push(json& j, const std::string& v) { j.push_back(v); }
X bool nj_has(const json& j, const std::string& k) { return j.contains(k); }
X json nj_copy(const json& j) { return j; }
X bool nj_eq(const json& a, const json& b) { return a == b; }
