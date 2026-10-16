//#region \0rolldown/runtime.js
var e = (e, t) => () => (t || (e((t = { exports: {} }).exports, t, t.exports), e = null), t.exports), t = /* @__PURE__ */ e(((e) => {
	Object.defineProperty(e, "__esModule", { value: !0 });
	function t(e) {
		let t = e.length, n = 0, r = 0, i;
		for (; r < t;) n++, i = e.charCodeAt(r++), i >= 55296 && i <= 56319 && r < t && (i = e.charCodeAt(r), (i & 64512) == 56320 && r++);
		return n;
	}
	e.default = t, t.code = "require(\"ajv/dist/runtime/ucs2length\").default";
})), n = (/* @__PURE__ */ e(((e) => {
	e.validateGraphDetails = i;
	var n = /* @__PURE__ */ RegExp("^(problem:.+|rule:.+|references|footnotes)$", "u"), r = t().default;
	function i(e, { instancePath: t = "", parentData: a, parentDataProperty: o, rootData: s = e, dynamicAnchors: c = {} } = {}) {
		let l = null, u = 0, d = i.evaluated;
		if (d.dynamicProps && (d.props = void 0), d.dynamicItems && (d.items = void 0), u === 0) {
			if (e && typeof e == "object" && !Array.isArray(e)) {
				let a;
				if (e.schema_version === void 0 && (a = "schema_version") || e.entries === void 0 && (a = "entries") || e.anchors === void 0 && (a = "anchors")) return i.errors = [{
					instancePath: t,
					schemaPath: "#/required",
					keyword: "required",
					params: { missingProperty: a },
					message: "must have required property '" + a + "'"
				}], !1;
				{
					let a = u;
					for (let n in e) if (n !== "schema_version" && n !== "entries" && n !== "anchors") return i.errors = [{
						instancePath: t,
						schemaPath: "#/additionalProperties",
						keyword: "additionalProperties",
						params: { additionalProperty: n },
						message: "must NOT have additional properties"
					}], !1;
					if (a === u) {
						if (e.schema_version !== void 0) {
							let n = e.schema_version, r = u;
							if (typeof n != "number" || n % 1 || isNaN(n) || !isFinite(n)) return i.errors = [{
								instancePath: t + "/schema_version",
								schemaPath: "#/properties/schema_version/type",
								keyword: "type",
								params: { type: "integer" },
								message: "must be integer"
							}], !1;
							if (n !== 1) return i.errors = [{
								instancePath: t + "/schema_version",
								schemaPath: "#/properties/schema_version/const",
								keyword: "const",
								params: { allowedValue: 1 },
								message: "must be equal to constant"
							}], !1;
							var f = r === u;
						} else var f = !0;
						if (f) {
							if (e.entries !== void 0) {
								let a = e.entries, o = u;
								if (u === o) {
									if (a && typeof a == "object" && !Array.isArray(a)) {
										if (Object.keys(a).length < 1) return i.errors = [{
											instancePath: t + "/entries",
											schemaPath: "#/properties/entries/minProperties",
											keyword: "minProperties",
											params: { limit: 1 },
											message: "must NOT have fewer than 1 properties"
										}], !1;
										for (let e in a) {
											let r = u;
											if (typeof e == "string" && !n.test(e)) {
												let n = {
													instancePath: t + "/entries",
													schemaPath: "#/properties/entries/propertyNames/pattern",
													keyword: "pattern",
													params: { pattern: "^(problem:.+|rule:.+|references|footnotes)$" },
													message: "must match pattern \"^(problem:.+|rule:.+|references|footnotes)$\"",
													propertyName: e
												};
												l === null ? l = [n] : l.push(n), u++;
											}
											var p = r === u;
											if (!p) {
												let n = {
													instancePath: t + "/entries",
													schemaPath: "#/properties/entries/propertyNames",
													keyword: "propertyNames",
													params: { propertyName: e },
													message: "property name must be valid"
												};
												return l === null ? l = [n] : l.push(n), u++, i.errors = l, !1;
											}
										}
										if (p) for (let e in a) {
											let n = a[e], o = u;
											if (u === o) {
												if (typeof n == "string") {
													if (r(n) < 1) return i.errors = [{
														instancePath: t + "/entries/" + e.replace(/~/g, "~0").replace(/\//g, "~1"),
														schemaPath: "#/properties/entries/additionalProperties/minLength",
														keyword: "minLength",
														params: { limit: 1 },
														message: "must NOT have fewer than 1 characters"
													}], !1;
												} else return i.errors = [{
													instancePath: t + "/entries/" + e.replace(/~/g, "~0").replace(/\//g, "~1"),
													schemaPath: "#/properties/entries/additionalProperties/type",
													keyword: "type",
													params: { type: "string" },
													message: "must be string"
												}], !1;
											}
											if (o !== u) break;
										}
									} else return i.errors = [{
										instancePath: t + "/entries",
										schemaPath: "#/properties/entries/type",
										keyword: "type",
										params: { type: "object" },
										message: "must be object"
									}], !1;
								}
								var f = o === u;
							} else var f = !0;
							if (f) {
								if (e.anchors !== void 0) {
									let n = e.anchors, a = u;
									if (u === a) {
										if (n && typeof n == "object" && !Array.isArray(n)) for (let e in n) {
											let a = n[e], o = u;
											if (u === o) {
												if (typeof a == "string") {
													if (r(a) < 1) return i.errors = [{
														instancePath: t + "/anchors/" + e.replace(/~/g, "~0").replace(/\//g, "~1"),
														schemaPath: "#/properties/anchors/additionalProperties/minLength",
														keyword: "minLength",
														params: { limit: 1 },
														message: "must NOT have fewer than 1 characters"
													}], !1;
												} else return i.errors = [{
													instancePath: t + "/anchors/" + e.replace(/~/g, "~0").replace(/\//g, "~1"),
													schemaPath: "#/properties/anchors/additionalProperties/type",
													keyword: "type",
													params: { type: "string" },
													message: "must be string"
												}], !1;
											}
											if (o !== u) break;
										}
										else return i.errors = [{
											instancePath: t + "/anchors",
											schemaPath: "#/properties/anchors/type",
											keyword: "type",
											params: { type: "object" },
											message: "must be object"
										}], !1;
									}
									var f = a === u;
								} else var f = !0;
							}
						}
					}
				}
			} else return i.errors = [{
				instancePath: t,
				schemaPath: "#/type",
				keyword: "type",
				params: { type: "object" },
				message: "must be object"
			}], !1;
		}
		return i.errors = l, u === 0;
	}
	i.evaluated = {
		props: !0,
		dynamicProps: !1,
		dynamicItems: !1
	};
})))().validateGraphDetails;
export { n as validateGraphDetails };
