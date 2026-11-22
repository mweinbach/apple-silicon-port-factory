// Reference instantiations of rapidjson for fndb library identification (features only).
#include "rapidjson/document.h"
#include "rapidjson/prettywriter.h"
#include "rapidjson/stringbuffer.h"
#include "rapidjson/writer.h"
#include "rapidjson/error/en.h"
#include <string>
#define X __declspec(dllexport)
using namespace rapidjson;
X bool rj_parse(Document& d, const char* s) { d.Parse(s); return !d.HasParseError(); }
X bool rj_parse_insitu(Document& d, char* s) { d.ParseInsitu(s); return !d.HasParseError(); }
X bool rj_parse_full(Document& d, const char* s) { d.Parse<kParseFullPrecisionFlag | kParseCommentsFlag | kParseTrailingCommasFlag | kParseNanAndInfFlag>(s); return !d.HasParseError(); }
X bool rj_parse_n(Document& d, const char* s, size_t n) { d.Parse(s, n); return !d.HasParseError(); }
X std::string rj_write(const Value& v) { StringBuffer b; Writer<StringBuffer> w(b); v.Accept(w); return b.GetString(); }
X std::string rj_pretty(const Value& v) { StringBuffer b; PrettyWriter<StringBuffer> w(b); v.Accept(w); return b.GetString(); }
X const char* rj_err(ParseErrorCode c) { return GetParseError_En(c); }
X const Value* rj_member(const Value& v, const char* k) { auto it = v.FindMember(k); return it == v.MemberEnd() ? nullptr : &it->value; }
X bool rj_has(const Value& v, const char* k) { return v.HasMember(k); }
X void rj_add(Document& d, Value& o, const char* k, int x) { o.AddMember(Value(k, d.GetAllocator()), Value(x), d.GetAllocator()); }
X void rj_push(Document& d, Value& a, double x) { a.PushBack(Value(x), d.GetAllocator()); }
X void rj_copy(Document& d, Value& dst, const Value& src) { dst.CopyFrom(src, d.GetAllocator()); }
X bool rj_eq(const Value& a, const Value& b) { return a == b; }
X double rj_getd(const Value& v) { return v.GetDouble(); }
X void rj_setstr(Document& d, Value& v, const char* s) { v.SetString(s, d.GetAllocator()); }
X void rj_remove(Value& v, const char* k) { v.RemoveMember(k); }
X void rj_erase(Value& v, const char* k) { v.EraseMember(k); }
X void rj_dtoa(double x, char* buf) { internal::dtoa(x, buf); }
X void rj_i64toa(long long x, char* buf) { internal::i64toa(x, buf); }
X void rj_u32toa(unsigned x, char* buf) { internal::u32toa(x, buf); }
X void rj_writer_d(Writer<StringBuffer>& w, double x) { w.Double(x); }
X void rj_writer_s(Writer<StringBuffer>& w, const char* s) { w.String(s); }
X void rj_writer_i(Writer<StringBuffer>& w, long long x) { w.Int64(x); }
X void rj_clear(Document& d) { d.SetObject(); }
X void rj_swap(Value& a, Value& b) { a.Swap(b); }
