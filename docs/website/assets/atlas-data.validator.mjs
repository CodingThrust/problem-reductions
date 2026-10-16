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
	e.validateAtlas = x;
	var n = {
		type: "string",
		enum: [
			"graph",
			"formula",
			"set",
			"algebraic",
			"misc"
		]
	}, r = /* @__PURE__ */ RegExp("^[A-Za-z_][A-Za-z0-9_]*$", "u"), i = t().default;
	function a(e, { instancePath: t = "", parentData: o, parentDataProperty: s, rootData: c = e, dynamicAnchors: l = {} } = {}) {
		let u = null, d = 0, f = a.evaluated;
		if (f.dynamicProps && (f.props = void 0), f.dynamicItems && (f.items = void 0), d === 0) {
			if (e && typeof e == "object" && !Array.isArray(e)) {
				let o;
				if (e.name === void 0 && (o = "name") || e.category === void 0 && (o = "category") || e.variant === void 0 && (o = "variant")) return a.errors = [{
					instancePath: t,
					schemaPath: "#/required",
					keyword: "required",
					params: { missingProperty: o },
					message: "must have required property '" + o + "'"
				}], !1;
				{
					let o = d;
					for (let n in e) if (n !== "name" && n !== "category" && n !== "variant") return a.errors = [{
						instancePath: t,
						schemaPath: "#/additionalProperties",
						keyword: "additionalProperties",
						params: { additionalProperty: n },
						message: "must NOT have additional properties"
					}], !1;
					if (o === d) {
						if (e.name !== void 0) {
							let n = e.name, i = d;
							if (d === d) {
								if (typeof n == "string") {
									if (!r.test(n)) return a.errors = [{
										instancePath: t + "/name",
										schemaPath: "#/$defs/Name/pattern",
										keyword: "pattern",
										params: { pattern: "^[A-Za-z_][A-Za-z0-9_]*$" },
										message: "must match pattern \"^[A-Za-z_][A-Za-z0-9_]*$\""
									}], !1;
								} else return a.errors = [{
									instancePath: t + "/name",
									schemaPath: "#/$defs/Name/type",
									keyword: "type",
									params: { type: "string" },
									message: "must be string"
								}], !1;
							}
							var p = i === d;
						} else var p = !0;
						if (p) {
							if (e.category !== void 0) {
								let r = e.category, i = d;
								if (typeof r != "string") return a.errors = [{
									instancePath: t + "/category",
									schemaPath: "#/$defs/Category/type",
									keyword: "type",
									params: { type: "string" },
									message: "must be string"
								}], !1;
								if (r !== "graph" && r !== "formula" && r !== "set" && r !== "algebraic" && r !== "misc") return a.errors = [{
									instancePath: t + "/category",
									schemaPath: "#/$defs/Category/enum",
									keyword: "enum",
									params: { allowedValues: n.enum },
									message: "must be equal to one of the allowed values"
								}], !1;
								var p = i === d;
							} else var p = !0;
							if (p) {
								if (e.variant !== void 0) {
									let n = e.variant, r = d;
									if (d === d) {
										if (n && typeof n == "object" && !Array.isArray(n)) {
											for (let e in n) {
												let n = d;
												if (typeof e == "string" && i(e) < 1) {
													let n = {
														instancePath: t + "/variant",
														schemaPath: "#/$defs/Variant/propertyNames/minLength",
														keyword: "minLength",
														params: { limit: 1 },
														message: "must NOT have fewer than 1 characters",
														propertyName: e
													};
													u === null ? u = [n] : u.push(n), d++;
												}
												var m = n === d;
												if (!m) {
													let n = {
														instancePath: t + "/variant",
														schemaPath: "#/$defs/Variant/propertyNames",
														keyword: "propertyNames",
														params: { propertyName: e },
														message: "property name must be valid"
													};
													return u === null ? u = [n] : u.push(n), d++, a.errors = u, !1;
												}
											}
											if (m) for (let e in n) {
												let r = n[e], o = d;
												if (d === o) {
													if (typeof r == "string") {
														if (i(r) < 1) return a.errors = [{
															instancePath: t + "/variant/" + e.replace(/~/g, "~0").replace(/\//g, "~1"),
															schemaPath: "#/$defs/Variant/additionalProperties/minLength",
															keyword: "minLength",
															params: { limit: 1 },
															message: "must NOT have fewer than 1 characters"
														}], !1;
													} else return a.errors = [{
														instancePath: t + "/variant/" + e.replace(/~/g, "~0").replace(/\//g, "~1"),
														schemaPath: "#/$defs/Variant/additionalProperties/type",
														keyword: "type",
														params: { type: "string" },
														message: "must be string"
													}], !1;
												}
												if (o !== d) break;
											}
										} else return a.errors = [{
											instancePath: t + "/variant",
											schemaPath: "#/$defs/Variant/type",
											keyword: "type",
											params: { type: "object" },
											message: "must be object"
										}], !1;
									}
									var p = r === d;
								} else var p = !0;
							}
						}
					}
				}
			} else return a.errors = [{
				instancePath: t,
				schemaPath: "#/type",
				keyword: "type",
				params: { type: "object" },
				message: "must be object"
			}], !1;
		}
		return a.errors = u, d === 0;
	}
	a.evaluated = {
		props: !0,
		dynamicProps: !1,
		dynamicItems: !1
	};
	function o(e, { instancePath: t = "", parentData: n, parentDataProperty: r, rootData: i = e, dynamicAnchors: a = {} } = {}) {
		let s = o.evaluated;
		if (s.dynamicProps && (s.props = void 0), s.dynamicItems && (s.items = void 0), e && typeof e == "object" && !Array.isArray(e)) {
			let n;
			if (e.source === void 0 && (n = "source") || e.target === void 0 && (n = "target")) return o.errors = [{
				instancePath: t,
				schemaPath: "#/required",
				keyword: "required",
				params: { missingProperty: n },
				message: "must have required property '" + n + "'"
			}], !1;
			for (let n in e) if (n !== "source" && n !== "target") return o.errors = [{
				instancePath: t,
				schemaPath: "#/additionalProperties",
				keyword: "additionalProperties",
				params: { additionalProperty: n },
				message: "must NOT have additional properties"
			}], !1;
			if (e.source !== void 0) {
				let n = e.source;
				if (typeof n != "number" || n % 1 || isNaN(n) || !isFinite(n)) return o.errors = [{
					instancePath: t + "/source",
					schemaPath: "#/$defs/Index/type",
					keyword: "type",
					params: { type: "integer" },
					message: "must be integer"
				}], !1;
				if (typeof n == "number" && isFinite(n)) {
					if (n > 9007199254740991 || isNaN(n)) return o.errors = [{
						instancePath: t + "/source",
						schemaPath: "#/$defs/Index/maximum",
						keyword: "maximum",
						params: {
							comparison: "<=",
							limit: 9007199254740991
						},
						message: "must be <= 9007199254740991"
					}], !1;
					if (n < 0 || isNaN(n)) return o.errors = [{
						instancePath: t + "/source",
						schemaPath: "#/$defs/Index/minimum",
						keyword: "minimum",
						params: {
							comparison: ">=",
							limit: 0
						},
						message: "must be >= 0"
					}], !1;
				}
				var c = !0;
			} else var c = !0;
			if (c) {
				if (e.target !== void 0) {
					let n = e.target;
					if (typeof n != "number" || n % 1 || isNaN(n) || !isFinite(n)) return o.errors = [{
						instancePath: t + "/target",
						schemaPath: "#/$defs/Index/type",
						keyword: "type",
						params: { type: "integer" },
						message: "must be integer"
					}], !1;
					if (typeof n == "number" && isFinite(n)) {
						if (n > 9007199254740991 || isNaN(n)) return o.errors = [{
							instancePath: t + "/target",
							schemaPath: "#/$defs/Index/maximum",
							keyword: "maximum",
							params: {
								comparison: "<=",
								limit: 9007199254740991
							},
							message: "must be <= 9007199254740991"
						}], !1;
						if (n < 0 || isNaN(n)) return o.errors = [{
							instancePath: t + "/target",
							schemaPath: "#/$defs/Index/minimum",
							keyword: "minimum",
							params: {
								comparison: ">=",
								limit: 0
							},
							message: "must be >= 0"
						}], !1;
					}
					var c = !0;
				} else var c = !0;
			}
		} else return o.errors = [{
			instancePath: t,
			schemaPath: "#/type",
			keyword: "type",
			params: { type: "object" },
			message: "must be object"
		}], !1;
		return o.errors = null, !0;
	}
	o.evaluated = {
		props: !0,
		dynamicProps: !1,
		dynamicItems: !1
	};
	function s(e, { instancePath: t = "", parentData: n, parentDataProperty: a, rootData: o = e, dynamicAnchors: c = {} } = {}) {
		let l = s.evaluated;
		if (l.dynamicProps && (l.props = void 0), l.dynamicItems && (l.items = void 0), e && typeof e == "object" && !Array.isArray(e)) {
			let n;
			if (e.name === void 0 && (n = "name") || e.description === void 0 && (n = "description")) return s.errors = [{
				instancePath: t,
				schemaPath: "#/required",
				keyword: "required",
				params: { missingProperty: n },
				message: "must have required property '" + n + "'"
			}], !1;
			for (let n in e) if (n !== "name" && n !== "description" && n !== "display_name" && n !== "aliases") return s.errors = [{
				instancePath: t,
				schemaPath: "#/additionalProperties",
				keyword: "additionalProperties",
				params: { additionalProperty: n },
				message: "must NOT have additional properties"
			}], !1;
			if (e.name !== void 0) {
				let n = e.name;
				if (typeof n == "string") {
					if (!r.test(n)) return s.errors = [{
						instancePath: t + "/name",
						schemaPath: "#/$defs/Name/pattern",
						keyword: "pattern",
						params: { pattern: "^[A-Za-z_][A-Za-z0-9_]*$" },
						message: "must match pattern \"^[A-Za-z_][A-Za-z0-9_]*$\""
					}], !1;
				} else return s.errors = [{
					instancePath: t + "/name",
					schemaPath: "#/$defs/Name/type",
					keyword: "type",
					params: { type: "string" },
					message: "must be string"
				}], !1;
				var u = !0;
			} else var u = !0;
			if (u) {
				if (e.description !== void 0) {
					if (typeof e.description != "string") return s.errors = [{
						instancePath: t + "/description",
						schemaPath: "#/properties/description/type",
						keyword: "type",
						params: { type: "string" },
						message: "must be string"
					}], !1;
					var u = !0;
				} else var u = !0;
				if (u) {
					if (e.display_name !== void 0) {
						let n = e.display_name;
						if (typeof n == "string") {
							if (i(n) < 1) return s.errors = [{
								instancePath: t + "/display_name",
								schemaPath: "#/properties/display_name/minLength",
								keyword: "minLength",
								params: { limit: 1 },
								message: "must NOT have fewer than 1 characters"
							}], !1;
						} else return s.errors = [{
							instancePath: t + "/display_name",
							schemaPath: "#/properties/display_name/type",
							keyword: "type",
							params: { type: "string" },
							message: "must be string"
						}], !1;
						var u = !0;
					} else var u = !0;
					if (u) {
						if (e.aliases !== void 0) {
							let n = e.aliases;
							if (Array.isArray(n)) {
								var d = !0;
								let e = n.length;
								for (let r = 0; r < e; r++) {
									let e = n[r];
									if (typeof e == "string") {
										if (i(e) < 1) return s.errors = [{
											instancePath: t + "/aliases/" + r,
											schemaPath: "#/properties/aliases/items/minLength",
											keyword: "minLength",
											params: { limit: 1 },
											message: "must NOT have fewer than 1 characters"
										}], !1;
									} else return s.errors = [{
										instancePath: t + "/aliases/" + r,
										schemaPath: "#/properties/aliases/items/type",
										keyword: "type",
										params: { type: "string" },
										message: "must be string"
									}], !1;
									var d = !0;
									if (!d) break;
								}
								if (d) {
									let e = n.length, r;
									if (e > 1) {
										let i = {};
										for (; e--;) {
											let a = n[e];
											if (typeof a == "string") {
												if (typeof i[a] == "number") return r = i[a], s.errors = [{
													instancePath: t + "/aliases",
													schemaPath: "#/properties/aliases/uniqueItems",
													keyword: "uniqueItems",
													params: {
														i: e,
														j: r
													},
													message: "must NOT have duplicate items (items ## " + r + " and " + e + " are identical)"
												}], !1;
												i[a] = e;
											}
										}
									}
								}
							} else return s.errors = [{
								instancePath: t + "/aliases",
								schemaPath: "#/properties/aliases/type",
								keyword: "type",
								params: { type: "array" },
								message: "must be array"
							}], !1;
							var u = !0;
						} else var u = !0;
					}
				}
			}
		} else return s.errors = [{
			instancePath: t,
			schemaPath: "#/type",
			keyword: "type",
			params: { type: "object" },
			message: "must be object"
		}], !1;
		return s.errors = null, !0;
	}
	s.evaluated = {
		props: !0,
		dynamicProps: !1,
		dynamicItems: !1
	};
	function c(e, { instancePath: t = "", parentData: n, parentDataProperty: r, rootData: i = e, dynamicAnchors: l = {} } = {}) {
		let u = null, d = 0, f = c.evaluated;
		if (f.dynamicProps && (f.props = void 0), f.dynamicItems && (f.items = void 0), d === 0) {
			if (e && typeof e == "object" && !Array.isArray(e)) {
				let n;
				if (e.schema_version === void 0 && (n = "schema_version") || e.kind === void 0 && (n = "kind") || e.nodes === void 0 && (n = "nodes") || e.edges === void 0 && (n = "edges") || e.schemas === void 0 && (n = "schemas")) return c.errors = [{
					instancePath: t,
					schemaPath: "#/required",
					keyword: "required",
					params: { missingProperty: n },
					message: "must have required property '" + n + "'"
				}], !1;
				{
					let n = d;
					for (let n in e) if (n !== "schema_version" && n !== "kind" && n !== "nodes" && n !== "edges" && n !== "schemas") return c.errors = [{
						instancePath: t,
						schemaPath: "#/additionalProperties",
						keyword: "additionalProperties",
						params: { additionalProperty: n },
						message: "must NOT have additional properties"
					}], !1;
					if (n === d) {
						if (e.schema_version !== void 0) {
							let n = e.schema_version, r = d;
							if (typeof n != "number" || n % 1 || isNaN(n) || !isFinite(n)) return c.errors = [{
								instancePath: t + "/schema_version",
								schemaPath: "#/properties/schema_version/type",
								keyword: "type",
								params: { type: "integer" },
								message: "must be integer"
							}], !1;
							if (n !== 1) return c.errors = [{
								instancePath: t + "/schema_version",
								schemaPath: "#/properties/schema_version/const",
								keyword: "const",
								params: { allowedValue: 1 },
								message: "must be equal to constant"
							}], !1;
							var p = r === d;
						} else var p = !0;
						if (p) {
							if (e.kind !== void 0) {
								let n = e.kind, r = d;
								if (typeof n != "string") return c.errors = [{
									instancePath: t + "/kind",
									schemaPath: "#/properties/kind/type",
									keyword: "type",
									params: { type: "string" },
									message: "must be string"
								}], !1;
								if (n !== "summary") return c.errors = [{
									instancePath: t + "/kind",
									schemaPath: "#/properties/kind/const",
									keyword: "const",
									params: { allowedValue: "summary" },
									message: "must be equal to constant"
								}], !1;
								var p = r === d;
							} else var p = !0;
							if (p) {
								if (e.nodes !== void 0) {
									let n = e.nodes, r = d;
									if (d === r) {
										if (Array.isArray(n)) {
											if (n.length < 1) return c.errors = [{
												instancePath: t + "/nodes",
												schemaPath: "#/properties/nodes/minItems",
												keyword: "minItems",
												params: { limit: 1 },
												message: "must NOT have fewer than 1 items"
											}], !1;
											{
												let e = n.length;
												for (let r = 0; r < e; r++) {
													let e = d;
													if (a(n[r], {
														instancePath: t + "/nodes/" + r,
														parentData: n,
														parentDataProperty: r,
														rootData: i,
														dynamicAnchors: l
													}) || (u = u === null ? a.errors : u.concat(a.errors), d = u.length), e !== d) break;
												}
											}
										} else return c.errors = [{
											instancePath: t + "/nodes",
											schemaPath: "#/properties/nodes/type",
											keyword: "type",
											params: { type: "array" },
											message: "must be array"
										}], !1;
									}
									var p = r === d;
								} else var p = !0;
								if (p) {
									if (e.edges !== void 0) {
										let n = e.edges, r = d;
										if (d === r) {
											if (Array.isArray(n)) {
												let e = n.length;
												for (let r = 0; r < e; r++) {
													let e = d;
													if (o(n[r], {
														instancePath: t + "/edges/" + r,
														parentData: n,
														parentDataProperty: r,
														rootData: i,
														dynamicAnchors: l
													}) || (u = u === null ? o.errors : u.concat(o.errors), d = u.length), e !== d) break;
												}
											} else return c.errors = [{
												instancePath: t + "/edges",
												schemaPath: "#/properties/edges/type",
												keyword: "type",
												params: { type: "array" },
												message: "must be array"
											}], !1;
										}
										var p = r === d;
									} else var p = !0;
									if (p) {
										if (e.schemas !== void 0) {
											let n = e.schemas, r = d;
											if (d === r) {
												if (Array.isArray(n)) {
													if (n.length < 1) return c.errors = [{
														instancePath: t + "/schemas",
														schemaPath: "#/properties/schemas/minItems",
														keyword: "minItems",
														params: { limit: 1 },
														message: "must NOT have fewer than 1 items"
													}], !1;
													{
														let e = n.length;
														for (let r = 0; r < e; r++) {
															let e = d;
															if (s(n[r], {
																instancePath: t + "/schemas/" + r,
																parentData: n,
																parentDataProperty: r,
																rootData: i,
																dynamicAnchors: l
															}) || (u = u === null ? s.errors : u.concat(s.errors), d = u.length), e !== d) break;
														}
													}
												} else return c.errors = [{
													instancePath: t + "/schemas",
													schemaPath: "#/properties/schemas/type",
													keyword: "type",
													params: { type: "array" },
													message: "must be array"
												}], !1;
											}
											var p = r === d;
										} else var p = !0;
									}
								}
							}
						}
					}
				}
			} else return c.errors = [{
				instancePath: t,
				schemaPath: "#/type",
				keyword: "type",
				params: { type: "object" },
				message: "must be object"
			}], !1;
		}
		return c.errors = u, d === 0;
	}
	c.evaluated = {
		props: !0,
		dynamicProps: !1,
		dynamicItems: !1
	};
	var l = {
		type: "object",
		additionalProperties: !1,
		required: [
			"name",
			"category",
			"variant",
			"default",
			"parameter_names",
			"complexity",
			"complexity_mathml",
			"doc_path",
			"api_path"
		],
		properties: {
			name: { $ref: "#/$defs/Name" },
			category: { $ref: "#/$defs/Category" },
			variant: { $ref: "#/$defs/Variant" },
			default: { type: "boolean" },
			parameter_names: {
				type: "array",
				items: {
					type: "string",
					minLength: 1
				},
				uniqueItems: !0
			},
			complexity: {
				type: "string",
				minLength: 1
			},
			complexity_mathml: { $ref: "#/$defs/MathML" },
			doc_path: { $ref: "#/$defs/DocPath" },
			api_path: { $ref: "#/$defs/DocPath" }
		}
	}, u = Object.prototype.hasOwnProperty, d = /* @__PURE__ */ RegExp("^<math xmlns=\"http://www\\.w3\\.org/1998/Math/MathML\" aria-label=\"[^\"<>]*\">(?:<(?:mrow|mi|mn|mo|mfrac|msup|msqrt)>|<mi mathvariant=\"normal\">|</(?:mrow|mi|mn|mo|mfrac|msup|msqrt)>|[^<>])*</math>$", "u"), f = /* @__PURE__ */ RegExp("^(?:[A-Za-z0-9_-]+/)*[A-Za-z0-9_.-]+\\.html$", "u");
	function p(e, { instancePath: t = "", parentData: a, parentDataProperty: o, rootData: s = e, dynamicAnchors: c = {} } = {}) {
		let m = null, h = 0, g = p.evaluated;
		if (g.dynamicProps && (g.props = void 0), g.dynamicItems && (g.items = void 0), h === 0) {
			if (e && typeof e == "object" && !Array.isArray(e)) {
				let a;
				if (e.name === void 0 && (a = "name") || e.category === void 0 && (a = "category") || e.variant === void 0 && (a = "variant") || e.default === void 0 && (a = "default") || e.parameter_names === void 0 && (a = "parameter_names") || e.complexity === void 0 && (a = "complexity") || e.complexity_mathml === void 0 && (a = "complexity_mathml") || e.doc_path === void 0 && (a = "doc_path") || e.api_path === void 0 && (a = "api_path")) return p.errors = [{
					instancePath: t,
					schemaPath: "#/required",
					keyword: "required",
					params: { missingProperty: a },
					message: "must have required property '" + a + "'"
				}], !1;
				{
					let a = h;
					for (let n in e) if (!u.call(l.properties, n)) return p.errors = [{
						instancePath: t,
						schemaPath: "#/additionalProperties",
						keyword: "additionalProperties",
						params: { additionalProperty: n },
						message: "must NOT have additional properties"
					}], !1;
					if (a === h) {
						if (e.name !== void 0) {
							let n = e.name, i = h;
							if (h === h) {
								if (typeof n == "string") {
									if (!r.test(n)) return p.errors = [{
										instancePath: t + "/name",
										schemaPath: "#/$defs/Name/pattern",
										keyword: "pattern",
										params: { pattern: "^[A-Za-z_][A-Za-z0-9_]*$" },
										message: "must match pattern \"^[A-Za-z_][A-Za-z0-9_]*$\""
									}], !1;
								} else return p.errors = [{
									instancePath: t + "/name",
									schemaPath: "#/$defs/Name/type",
									keyword: "type",
									params: { type: "string" },
									message: "must be string"
								}], !1;
							}
							var _ = i === h;
						} else var _ = !0;
						if (_) {
							if (e.category !== void 0) {
								let r = e.category, i = h;
								if (typeof r != "string") return p.errors = [{
									instancePath: t + "/category",
									schemaPath: "#/$defs/Category/type",
									keyword: "type",
									params: { type: "string" },
									message: "must be string"
								}], !1;
								if (r !== "graph" && r !== "formula" && r !== "set" && r !== "algebraic" && r !== "misc") return p.errors = [{
									instancePath: t + "/category",
									schemaPath: "#/$defs/Category/enum",
									keyword: "enum",
									params: { allowedValues: n.enum },
									message: "must be equal to one of the allowed values"
								}], !1;
								var _ = i === h;
							} else var _ = !0;
							if (_) {
								if (e.variant !== void 0) {
									let n = e.variant, r = h;
									if (h === h) {
										if (n && typeof n == "object" && !Array.isArray(n)) {
											for (let e in n) {
												let n = h;
												if (typeof e == "string" && i(e) < 1) {
													let n = {
														instancePath: t + "/variant",
														schemaPath: "#/$defs/Variant/propertyNames/minLength",
														keyword: "minLength",
														params: { limit: 1 },
														message: "must NOT have fewer than 1 characters",
														propertyName: e
													};
													m === null ? m = [n] : m.push(n), h++;
												}
												var v = n === h;
												if (!v) {
													let n = {
														instancePath: t + "/variant",
														schemaPath: "#/$defs/Variant/propertyNames",
														keyword: "propertyNames",
														params: { propertyName: e },
														message: "property name must be valid"
													};
													return m === null ? m = [n] : m.push(n), h++, p.errors = m, !1;
												}
											}
											if (v) for (let e in n) {
												let r = n[e], a = h;
												if (h === a) {
													if (typeof r == "string") {
														if (i(r) < 1) return p.errors = [{
															instancePath: t + "/variant/" + e.replace(/~/g, "~0").replace(/\//g, "~1"),
															schemaPath: "#/$defs/Variant/additionalProperties/minLength",
															keyword: "minLength",
															params: { limit: 1 },
															message: "must NOT have fewer than 1 characters"
														}], !1;
													} else return p.errors = [{
														instancePath: t + "/variant/" + e.replace(/~/g, "~0").replace(/\//g, "~1"),
														schemaPath: "#/$defs/Variant/additionalProperties/type",
														keyword: "type",
														params: { type: "string" },
														message: "must be string"
													}], !1;
												}
												if (a !== h) break;
											}
										} else return p.errors = [{
											instancePath: t + "/variant",
											schemaPath: "#/$defs/Variant/type",
											keyword: "type",
											params: { type: "object" },
											message: "must be object"
										}], !1;
									}
									var _ = r === h;
								} else var _ = !0;
								if (_) {
									if (e.default !== void 0) {
										let n = h;
										if (typeof e.default != "boolean") return p.errors = [{
											instancePath: t + "/default",
											schemaPath: "#/properties/default/type",
											keyword: "type",
											params: { type: "boolean" },
											message: "must be boolean"
										}], !1;
										var _ = n === h;
									} else var _ = !0;
									if (_) {
										if (e.parameter_names !== void 0) {
											let n = e.parameter_names, r = h;
											if (h === r) {
												if (Array.isArray(n)) {
													var y = !0;
													let e = n.length;
													for (let r = 0; r < e; r++) {
														let e = n[r], a = h;
														if (h === a) {
															if (typeof e == "string") {
																if (i(e) < 1) return p.errors = [{
																	instancePath: t + "/parameter_names/" + r,
																	schemaPath: "#/properties/parameter_names/items/minLength",
																	keyword: "minLength",
																	params: { limit: 1 },
																	message: "must NOT have fewer than 1 characters"
																}], !1;
															} else return p.errors = [{
																instancePath: t + "/parameter_names/" + r,
																schemaPath: "#/properties/parameter_names/items/type",
																keyword: "type",
																params: { type: "string" },
																message: "must be string"
															}], !1;
														}
														var y = a === h;
														if (!y) break;
													}
													if (y) {
														let e = n.length, r;
														if (e > 1) {
															let i = {};
															for (; e--;) {
																let a = n[e];
																if (typeof a == "string") {
																	if (typeof i[a] == "number") return r = i[a], p.errors = [{
																		instancePath: t + "/parameter_names",
																		schemaPath: "#/properties/parameter_names/uniqueItems",
																		keyword: "uniqueItems",
																		params: {
																			i: e,
																			j: r
																		},
																		message: "must NOT have duplicate items (items ## " + r + " and " + e + " are identical)"
																	}], !1;
																	i[a] = e;
																}
															}
														}
													}
												} else return p.errors = [{
													instancePath: t + "/parameter_names",
													schemaPath: "#/properties/parameter_names/type",
													keyword: "type",
													params: { type: "array" },
													message: "must be array"
												}], !1;
											}
											var _ = r === h;
										} else var _ = !0;
										if (_) {
											if (e.complexity !== void 0) {
												let n = e.complexity, r = h;
												if (h === r) {
													if (typeof n == "string") {
														if (i(n) < 1) return p.errors = [{
															instancePath: t + "/complexity",
															schemaPath: "#/properties/complexity/minLength",
															keyword: "minLength",
															params: { limit: 1 },
															message: "must NOT have fewer than 1 characters"
														}], !1;
													} else return p.errors = [{
														instancePath: t + "/complexity",
														schemaPath: "#/properties/complexity/type",
														keyword: "type",
														params: { type: "string" },
														message: "must be string"
													}], !1;
												}
												var _ = r === h;
											} else var _ = !0;
											if (_) {
												if (e.complexity_mathml !== void 0) {
													let n = e.complexity_mathml, r = h;
													if (h === h) {
														if (typeof n == "string") {
															if (!d.test(n)) return p.errors = [{
																instancePath: t + "/complexity_mathml",
																schemaPath: "#/$defs/MathML/pattern",
																keyword: "pattern",
																params: { pattern: "^<math xmlns=\"http://www\\.w3\\.org/1998/Math/MathML\" aria-label=\"[^\"<>]*\">(?:<(?:mrow|mi|mn|mo|mfrac|msup|msqrt)>|<mi mathvariant=\"normal\">|</(?:mrow|mi|mn|mo|mfrac|msup|msqrt)>|[^<>])*</math>$" },
																message: "must match pattern \"^<math xmlns=\"http://www\\.w3\\.org/1998/Math/MathML\" aria-label=\"[^\"<>]*\">(?:<(?:mrow|mi|mn|mo|mfrac|msup|msqrt)>|<mi mathvariant=\"normal\">|</(?:mrow|mi|mn|mo|mfrac|msup|msqrt)>|[^<>])*</math>$\""
															}], !1;
														} else return p.errors = [{
															instancePath: t + "/complexity_mathml",
															schemaPath: "#/$defs/MathML/type",
															keyword: "type",
															params: { type: "string" },
															message: "must be string"
														}], !1;
													}
													var _ = r === h;
												} else var _ = !0;
												if (_) {
													if (e.doc_path !== void 0) {
														let n = e.doc_path, r = h;
														if (h === h) {
															if (typeof n == "string") {
																if (!f.test(n)) return p.errors = [{
																	instancePath: t + "/doc_path",
																	schemaPath: "#/$defs/DocPath/pattern",
																	keyword: "pattern",
																	params: { pattern: "^(?:[A-Za-z0-9_-]+/)*[A-Za-z0-9_.-]+\\.html$" },
																	message: "must match pattern \"^(?:[A-Za-z0-9_-]+/)*[A-Za-z0-9_.-]+\\.html$\""
																}], !1;
															} else return p.errors = [{
																instancePath: t + "/doc_path",
																schemaPath: "#/$defs/DocPath/type",
																keyword: "type",
																params: { type: "string" },
																message: "must be string"
															}], !1;
														}
														var _ = r === h;
													} else var _ = !0;
													if (_) {
														if (e.api_path !== void 0) {
															let n = e.api_path, r = h;
															if (h === h) {
																if (typeof n == "string") {
																	if (!f.test(n)) return p.errors = [{
																		instancePath: t + "/api_path",
																		schemaPath: "#/$defs/DocPath/pattern",
																		keyword: "pattern",
																		params: { pattern: "^(?:[A-Za-z0-9_-]+/)*[A-Za-z0-9_.-]+\\.html$" },
																		message: "must match pattern \"^(?:[A-Za-z0-9_-]+/)*[A-Za-z0-9_.-]+\\.html$\""
																	}], !1;
																} else return p.errors = [{
																	instancePath: t + "/api_path",
																	schemaPath: "#/$defs/DocPath/type",
																	keyword: "type",
																	params: { type: "string" },
																	message: "must be string"
																}], !1;
															}
															var _ = r === h;
														} else var _ = !0;
													}
												}
											}
										}
									}
								}
							}
						}
					}
				}
			} else return p.errors = [{
				instancePath: t,
				schemaPath: "#/type",
				keyword: "type",
				params: { type: "object" },
				message: "must be object"
			}], !1;
		}
		return p.errors = m, h === 0;
	}
	p.evaluated = {
		props: !0,
		dynamicProps: !1,
		dynamicItems: !1
	};
	var m = {
		type: "object",
		additionalProperties: !1,
		required: [
			"source",
			"target",
			"witness",
			"aggregate",
			"turing",
			"parameter_contract_error",
			"parameters",
			"doc_path",
			"source_path",
			"api_path",
			"test_path"
		],
		properties: {
			source: { $ref: "#/$defs/Index" },
			target: { $ref: "#/$defs/Index" },
			witness: { type: "boolean" },
			aggregate: { type: "boolean" },
			turing: { type: "boolean" },
			parameter_contract_error: {
				type: ["string", "null"],
				minLength: 1
			},
			parameters: {
				type: "array",
				items: { $ref: "#/$defs/Parameter" }
			},
			doc_path: { $ref: "#/$defs/DocPath" },
			source_path: { $ref: "#/$defs/SourcePath" },
			api_path: { $ref: "#/$defs/DocPath" },
			test_path: { anyOf: [{ $ref: "#/$defs/SourcePath" }, { type: "null" }] }
		}
	}, h = { oneOf: [{
		type: "object",
		additionalProperties: !1,
		required: [
			"field",
			"contract",
			"formula",
			"reason",
			"mathml"
		],
		properties: {
			field: {
				type: "string",
				minLength: 1
			},
			contract: {
				type: "string",
				enum: ["exact", "upper_bound"]
			},
			formula: {
				type: "string",
				minLength: 1
			},
			reason: { type: "null" },
			mathml: { $ref: "#/$defs/MathML" }
		}
	}, {
		type: "object",
		additionalProperties: !1,
		required: [
			"field",
			"contract",
			"formula",
			"reason",
			"mathml"
		],
		properties: {
			field: {
				type: "string",
				minLength: 1
			},
			contract: {
				type: "string",
				const: "unavailable"
			},
			formula: { type: "null" },
			reason: {
				type: "string",
				minLength: 1
			},
			mathml: { type: "null" }
		}
	}] };
	function g(e, { instancePath: t = "", parentData: n, parentDataProperty: r, rootData: a = e, dynamicAnchors: o = {} } = {}) {
		let s = null, c = 0, l = g.evaluated;
		l.dynamicProps && (l.props = void 0), l.dynamicItems && (l.items = void 0);
		let u = c, f = !1, p = null, m = c;
		if (c === m) {
			if (e && typeof e == "object" && !Array.isArray(e)) {
				let n;
				if (e.field === void 0 && (n = "field") || e.contract === void 0 && (n = "contract") || e.formula === void 0 && (n = "formula") || e.reason === void 0 && (n = "reason") || e.mathml === void 0 && (n = "mathml")) {
					let e = {
						instancePath: t,
						schemaPath: "#/oneOf/0/required",
						keyword: "required",
						params: { missingProperty: n },
						message: "must have required property '" + n + "'"
					};
					s === null ? s = [e] : s.push(e), c++;
				} else {
					let n = c;
					for (let n in e) if (n !== "field" && n !== "contract" && n !== "formula" && n !== "reason" && n !== "mathml") {
						let e = {
							instancePath: t,
							schemaPath: "#/oneOf/0/additionalProperties",
							keyword: "additionalProperties",
							params: { additionalProperty: n },
							message: "must NOT have additional properties"
						};
						s === null ? s = [e] : s.push(e), c++;
						break;
					}
					if (n === c) {
						if (e.field !== void 0) {
							let n = e.field, r = c;
							if (c === r) {
								if (typeof n == "string") {
									if (i(n) < 1) {
										let e = {
											instancePath: t + "/field",
											schemaPath: "#/oneOf/0/properties/field/minLength",
											keyword: "minLength",
											params: { limit: 1 },
											message: "must NOT have fewer than 1 characters"
										};
										s === null ? s = [e] : s.push(e), c++;
									}
								} else {
									let e = {
										instancePath: t + "/field",
										schemaPath: "#/oneOf/0/properties/field/type",
										keyword: "type",
										params: { type: "string" },
										message: "must be string"
									};
									s === null ? s = [e] : s.push(e), c++;
								}
							}
							var _ = r === c;
						} else var _ = !0;
						if (_) {
							if (e.contract !== void 0) {
								let n = e.contract, r = c;
								if (typeof n != "string") {
									let e = {
										instancePath: t + "/contract",
										schemaPath: "#/oneOf/0/properties/contract/type",
										keyword: "type",
										params: { type: "string" },
										message: "must be string"
									};
									s === null ? s = [e] : s.push(e), c++;
								}
								if (n !== "exact" && n !== "upper_bound") {
									let e = {
										instancePath: t + "/contract",
										schemaPath: "#/oneOf/0/properties/contract/enum",
										keyword: "enum",
										params: { allowedValues: h.oneOf[0].properties.contract.enum },
										message: "must be equal to one of the allowed values"
									};
									s === null ? s = [e] : s.push(e), c++;
								}
								var _ = r === c;
							} else var _ = !0;
							if (_) {
								if (e.formula !== void 0) {
									let n = e.formula, r = c;
									if (c === r) {
										if (typeof n == "string") {
											if (i(n) < 1) {
												let e = {
													instancePath: t + "/formula",
													schemaPath: "#/oneOf/0/properties/formula/minLength",
													keyword: "minLength",
													params: { limit: 1 },
													message: "must NOT have fewer than 1 characters"
												};
												s === null ? s = [e] : s.push(e), c++;
											}
										} else {
											let e = {
												instancePath: t + "/formula",
												schemaPath: "#/oneOf/0/properties/formula/type",
												keyword: "type",
												params: { type: "string" },
												message: "must be string"
											};
											s === null ? s = [e] : s.push(e), c++;
										}
									}
									var _ = r === c;
								} else var _ = !0;
								if (_) {
									if (e.reason !== void 0) {
										let n = c;
										if (e.reason !== null) {
											let e = {
												instancePath: t + "/reason",
												schemaPath: "#/oneOf/0/properties/reason/type",
												keyword: "type",
												params: { type: "null" },
												message: "must be null"
											};
											s === null ? s = [e] : s.push(e), c++;
										}
										var _ = n === c;
									} else var _ = !0;
									if (_) {
										if (e.mathml !== void 0) {
											let n = e.mathml, r = c;
											if (c === c) {
												if (typeof n == "string") {
													if (!d.test(n)) {
														let e = {
															instancePath: t + "/mathml",
															schemaPath: "#/$defs/MathML/pattern",
															keyword: "pattern",
															params: { pattern: "^<math xmlns=\"http://www\\.w3\\.org/1998/Math/MathML\" aria-label=\"[^\"<>]*\">(?:<(?:mrow|mi|mn|mo|mfrac|msup|msqrt)>|<mi mathvariant=\"normal\">|</(?:mrow|mi|mn|mo|mfrac|msup|msqrt)>|[^<>])*</math>$" },
															message: "must match pattern \"^<math xmlns=\"http://www\\.w3\\.org/1998/Math/MathML\" aria-label=\"[^\"<>]*\">(?:<(?:mrow|mi|mn|mo|mfrac|msup|msqrt)>|<mi mathvariant=\"normal\">|</(?:mrow|mi|mn|mo|mfrac|msup|msqrt)>|[^<>])*</math>$\""
														};
														s === null ? s = [e] : s.push(e), c++;
													}
												} else {
													let e = {
														instancePath: t + "/mathml",
														schemaPath: "#/$defs/MathML/type",
														keyword: "type",
														params: { type: "string" },
														message: "must be string"
													};
													s === null ? s = [e] : s.push(e), c++;
												}
											}
											var _ = r === c;
										} else var _ = !0;
									}
								}
							}
						}
					}
				}
			} else {
				let e = {
					instancePath: t,
					schemaPath: "#/oneOf/0/type",
					keyword: "type",
					params: { type: "object" },
					message: "must be object"
				};
				s === null ? s = [e] : s.push(e), c++;
			}
		}
		var v = m === c;
		if (v) {
			f = !0, p = 0;
			var y = !0;
		}
		let b = c;
		if (c === b) {
			if (e && typeof e == "object" && !Array.isArray(e)) {
				let n;
				if (e.field === void 0 && (n = "field") || e.contract === void 0 && (n = "contract") || e.formula === void 0 && (n = "formula") || e.reason === void 0 && (n = "reason") || e.mathml === void 0 && (n = "mathml")) {
					let e = {
						instancePath: t,
						schemaPath: "#/oneOf/1/required",
						keyword: "required",
						params: { missingProperty: n },
						message: "must have required property '" + n + "'"
					};
					s === null ? s = [e] : s.push(e), c++;
				} else {
					let n = c;
					for (let n in e) if (n !== "field" && n !== "contract" && n !== "formula" && n !== "reason" && n !== "mathml") {
						let e = {
							instancePath: t,
							schemaPath: "#/oneOf/1/additionalProperties",
							keyword: "additionalProperties",
							params: { additionalProperty: n },
							message: "must NOT have additional properties"
						};
						s === null ? s = [e] : s.push(e), c++;
						break;
					}
					if (n === c) {
						if (e.field !== void 0) {
							let n = e.field, r = c;
							if (c === r) {
								if (typeof n == "string") {
									if (i(n) < 1) {
										let e = {
											instancePath: t + "/field",
											schemaPath: "#/oneOf/1/properties/field/minLength",
											keyword: "minLength",
											params: { limit: 1 },
											message: "must NOT have fewer than 1 characters"
										};
										s === null ? s = [e] : s.push(e), c++;
									}
								} else {
									let e = {
										instancePath: t + "/field",
										schemaPath: "#/oneOf/1/properties/field/type",
										keyword: "type",
										params: { type: "string" },
										message: "must be string"
									};
									s === null ? s = [e] : s.push(e), c++;
								}
							}
							var x = r === c;
						} else var x = !0;
						if (x) {
							if (e.contract !== void 0) {
								let n = e.contract, r = c;
								if (typeof n != "string") {
									let e = {
										instancePath: t + "/contract",
										schemaPath: "#/oneOf/1/properties/contract/type",
										keyword: "type",
										params: { type: "string" },
										message: "must be string"
									};
									s === null ? s = [e] : s.push(e), c++;
								}
								if (n !== "unavailable") {
									let e = {
										instancePath: t + "/contract",
										schemaPath: "#/oneOf/1/properties/contract/const",
										keyword: "const",
										params: { allowedValue: "unavailable" },
										message: "must be equal to constant"
									};
									s === null ? s = [e] : s.push(e), c++;
								}
								var x = r === c;
							} else var x = !0;
							if (x) {
								if (e.formula !== void 0) {
									let n = c;
									if (e.formula !== null) {
										let e = {
											instancePath: t + "/formula",
											schemaPath: "#/oneOf/1/properties/formula/type",
											keyword: "type",
											params: { type: "null" },
											message: "must be null"
										};
										s === null ? s = [e] : s.push(e), c++;
									}
									var x = n === c;
								} else var x = !0;
								if (x) {
									if (e.reason !== void 0) {
										let n = e.reason, r = c;
										if (c === r) {
											if (typeof n == "string") {
												if (i(n) < 1) {
													let e = {
														instancePath: t + "/reason",
														schemaPath: "#/oneOf/1/properties/reason/minLength",
														keyword: "minLength",
														params: { limit: 1 },
														message: "must NOT have fewer than 1 characters"
													};
													s === null ? s = [e] : s.push(e), c++;
												}
											} else {
												let e = {
													instancePath: t + "/reason",
													schemaPath: "#/oneOf/1/properties/reason/type",
													keyword: "type",
													params: { type: "string" },
													message: "must be string"
												};
												s === null ? s = [e] : s.push(e), c++;
											}
										}
										var x = r === c;
									} else var x = !0;
									if (x) {
										if (e.mathml !== void 0) {
											let n = c;
											if (e.mathml !== null) {
												let e = {
													instancePath: t + "/mathml",
													schemaPath: "#/oneOf/1/properties/mathml/type",
													keyword: "type",
													params: { type: "null" },
													message: "must be null"
												};
												s === null ? s = [e] : s.push(e), c++;
											}
											var x = n === c;
										} else var x = !0;
									}
								}
							}
						}
					}
				}
			} else {
				let e = {
					instancePath: t,
					schemaPath: "#/oneOf/1/type",
					keyword: "type",
					params: { type: "object" },
					message: "must be object"
				};
				s === null ? s = [e] : s.push(e), c++;
			}
		}
		var v = b === c;
		if (v && f ? (f = !1, p = [p, 1]) : v && (f = !0, p = 1, y !== !0 && (y = !0)), f) c = u, s !== null && (u ? s.length = u : s = null);
		else {
			let e = {
				instancePath: t,
				schemaPath: "#/oneOf",
				keyword: "oneOf",
				params: { passingSchemas: p },
				message: "must match exactly one schema in oneOf"
			};
			return s === null ? s = [e] : s.push(e), c++, g.errors = s, !1;
		}
		return g.errors = s, l.props = y, c === 0;
	}
	g.evaluated = {
		dynamicProps: !0,
		dynamicItems: !1
	};
	var _ = /* @__PURE__ */ RegExp("^src/(?:[A-Za-z0-9_-]+/)*[A-Za-z0-9_-]+\\.rs$", "u");
	function v(e, { instancePath: t = "", parentData: n, parentDataProperty: r, rootData: a = e, dynamicAnchors: o = {} } = {}) {
		let s = null, c = 0, l = v.evaluated;
		if (l.dynamicProps && (l.props = void 0), l.dynamicItems && (l.items = void 0), c === 0) {
			if (e && typeof e == "object" && !Array.isArray(e)) {
				let n;
				if (e.source === void 0 && (n = "source") || e.target === void 0 && (n = "target") || e.witness === void 0 && (n = "witness") || e.aggregate === void 0 && (n = "aggregate") || e.turing === void 0 && (n = "turing") || e.parameter_contract_error === void 0 && (n = "parameter_contract_error") || e.parameters === void 0 && (n = "parameters") || e.doc_path === void 0 && (n = "doc_path") || e.source_path === void 0 && (n = "source_path") || e.api_path === void 0 && (n = "api_path") || e.test_path === void 0 && (n = "test_path")) return v.errors = [{
					instancePath: t,
					schemaPath: "#/required",
					keyword: "required",
					params: { missingProperty: n },
					message: "must have required property '" + n + "'"
				}], !1;
				{
					let n = c;
					for (let n in e) if (!u.call(m.properties, n)) return v.errors = [{
						instancePath: t,
						schemaPath: "#/additionalProperties",
						keyword: "additionalProperties",
						params: { additionalProperty: n },
						message: "must NOT have additional properties"
					}], !1;
					if (n === c) {
						if (e.source !== void 0) {
							let n = e.source, r = c, i = c;
							if (typeof n != "number" || n % 1 || isNaN(n) || !isFinite(n)) return v.errors = [{
								instancePath: t + "/source",
								schemaPath: "#/$defs/Index/type",
								keyword: "type",
								params: { type: "integer" },
								message: "must be integer"
							}], !1;
							if (c === i && typeof n == "number" && isFinite(n)) {
								if (n > 9007199254740991 || isNaN(n)) return v.errors = [{
									instancePath: t + "/source",
									schemaPath: "#/$defs/Index/maximum",
									keyword: "maximum",
									params: {
										comparison: "<=",
										limit: 9007199254740991
									},
									message: "must be <= 9007199254740991"
								}], !1;
								if (n < 0 || isNaN(n)) return v.errors = [{
									instancePath: t + "/source",
									schemaPath: "#/$defs/Index/minimum",
									keyword: "minimum",
									params: {
										comparison: ">=",
										limit: 0
									},
									message: "must be >= 0"
								}], !1;
							}
							var d = r === c;
						} else var d = !0;
						if (d) {
							if (e.target !== void 0) {
								let n = e.target, r = c, i = c;
								if (typeof n != "number" || n % 1 || isNaN(n) || !isFinite(n)) return v.errors = [{
									instancePath: t + "/target",
									schemaPath: "#/$defs/Index/type",
									keyword: "type",
									params: { type: "integer" },
									message: "must be integer"
								}], !1;
								if (c === i && typeof n == "number" && isFinite(n)) {
									if (n > 9007199254740991 || isNaN(n)) return v.errors = [{
										instancePath: t + "/target",
										schemaPath: "#/$defs/Index/maximum",
										keyword: "maximum",
										params: {
											comparison: "<=",
											limit: 9007199254740991
										},
										message: "must be <= 9007199254740991"
									}], !1;
									if (n < 0 || isNaN(n)) return v.errors = [{
										instancePath: t + "/target",
										schemaPath: "#/$defs/Index/minimum",
										keyword: "minimum",
										params: {
											comparison: ">=",
											limit: 0
										},
										message: "must be >= 0"
									}], !1;
								}
								var d = r === c;
							} else var d = !0;
							if (d) {
								if (e.witness !== void 0) {
									let n = c;
									if (typeof e.witness != "boolean") return v.errors = [{
										instancePath: t + "/witness",
										schemaPath: "#/properties/witness/type",
										keyword: "type",
										params: { type: "boolean" },
										message: "must be boolean"
									}], !1;
									var d = n === c;
								} else var d = !0;
								if (d) {
									if (e.aggregate !== void 0) {
										let n = c;
										if (typeof e.aggregate != "boolean") return v.errors = [{
											instancePath: t + "/aggregate",
											schemaPath: "#/properties/aggregate/type",
											keyword: "type",
											params: { type: "boolean" },
											message: "must be boolean"
										}], !1;
										var d = n === c;
									} else var d = !0;
									if (d) {
										if (e.turing !== void 0) {
											let n = c;
											if (typeof e.turing != "boolean") return v.errors = [{
												instancePath: t + "/turing",
												schemaPath: "#/properties/turing/type",
												keyword: "type",
												params: { type: "boolean" },
												message: "must be boolean"
											}], !1;
											var d = n === c;
										} else var d = !0;
										if (d) {
											if (e.parameter_contract_error !== void 0) {
												let n = e.parameter_contract_error, r = c;
												if (typeof n != "string" && n !== null) return v.errors = [{
													instancePath: t + "/parameter_contract_error",
													schemaPath: "#/properties/parameter_contract_error/type",
													keyword: "type",
													params: { type: m.properties.parameter_contract_error.type },
													message: "must be string,null"
												}], !1;
												if (c === r && typeof n == "string" && i(n) < 1) return v.errors = [{
													instancePath: t + "/parameter_contract_error",
													schemaPath: "#/properties/parameter_contract_error/minLength",
													keyword: "minLength",
													params: { limit: 1 },
													message: "must NOT have fewer than 1 characters"
												}], !1;
												var d = r === c;
											} else var d = !0;
											if (d) {
												if (e.parameters !== void 0) {
													let n = e.parameters, r = c;
													if (c === r) {
														if (Array.isArray(n)) {
															let e = n.length;
															for (let r = 0; r < e; r++) {
																let e = c;
																if (g(n[r], {
																	instancePath: t + "/parameters/" + r,
																	parentData: n,
																	parentDataProperty: r,
																	rootData: a,
																	dynamicAnchors: o
																}) || (s = s === null ? g.errors : s.concat(g.errors), c = s.length), e !== c) break;
															}
														} else return v.errors = [{
															instancePath: t + "/parameters",
															schemaPath: "#/properties/parameters/type",
															keyword: "type",
															params: { type: "array" },
															message: "must be array"
														}], !1;
													}
													var d = r === c;
												} else var d = !0;
												if (d) {
													if (e.doc_path !== void 0) {
														let n = e.doc_path, r = c;
														if (c === c) {
															if (typeof n == "string") {
																if (!f.test(n)) return v.errors = [{
																	instancePath: t + "/doc_path",
																	schemaPath: "#/$defs/DocPath/pattern",
																	keyword: "pattern",
																	params: { pattern: "^(?:[A-Za-z0-9_-]+/)*[A-Za-z0-9_.-]+\\.html$" },
																	message: "must match pattern \"^(?:[A-Za-z0-9_-]+/)*[A-Za-z0-9_.-]+\\.html$\""
																}], !1;
															} else return v.errors = [{
																instancePath: t + "/doc_path",
																schemaPath: "#/$defs/DocPath/type",
																keyword: "type",
																params: { type: "string" },
																message: "must be string"
															}], !1;
														}
														var d = r === c;
													} else var d = !0;
													if (d) {
														if (e.source_path !== void 0) {
															let n = e.source_path, r = c;
															if (c === c) {
																if (typeof n == "string") {
																	if (!_.test(n)) return v.errors = [{
																		instancePath: t + "/source_path",
																		schemaPath: "#/$defs/SourcePath/pattern",
																		keyword: "pattern",
																		params: { pattern: "^src/(?:[A-Za-z0-9_-]+/)*[A-Za-z0-9_-]+\\.rs$" },
																		message: "must match pattern \"^src/(?:[A-Za-z0-9_-]+/)*[A-Za-z0-9_-]+\\.rs$\""
																	}], !1;
																} else return v.errors = [{
																	instancePath: t + "/source_path",
																	schemaPath: "#/$defs/SourcePath/type",
																	keyword: "type",
																	params: { type: "string" },
																	message: "must be string"
																}], !1;
															}
															var d = r === c;
														} else var d = !0;
														if (d) {
															if (e.api_path !== void 0) {
																let n = e.api_path, r = c;
																if (c === c) {
																	if (typeof n == "string") {
																		if (!f.test(n)) return v.errors = [{
																			instancePath: t + "/api_path",
																			schemaPath: "#/$defs/DocPath/pattern",
																			keyword: "pattern",
																			params: { pattern: "^(?:[A-Za-z0-9_-]+/)*[A-Za-z0-9_.-]+\\.html$" },
																			message: "must match pattern \"^(?:[A-Za-z0-9_-]+/)*[A-Za-z0-9_.-]+\\.html$\""
																		}], !1;
																	} else return v.errors = [{
																		instancePath: t + "/api_path",
																		schemaPath: "#/$defs/DocPath/type",
																		keyword: "type",
																		params: { type: "string" },
																		message: "must be string"
																	}], !1;
																}
																var d = r === c;
															} else var d = !0;
															if (d) {
																if (e.test_path !== void 0) {
																	let n = e.test_path, r = c, i = c, a = !1, o = c;
																	if (c === c) {
																		if (typeof n == "string") {
																			if (!_.test(n)) {
																				let e = {
																					instancePath: t + "/test_path",
																					schemaPath: "#/$defs/SourcePath/pattern",
																					keyword: "pattern",
																					params: { pattern: "^src/(?:[A-Za-z0-9_-]+/)*[A-Za-z0-9_-]+\\.rs$" },
																					message: "must match pattern \"^src/(?:[A-Za-z0-9_-]+/)*[A-Za-z0-9_-]+\\.rs$\""
																				};
																				s === null ? s = [e] : s.push(e), c++;
																			}
																		} else {
																			let e = {
																				instancePath: t + "/test_path",
																				schemaPath: "#/$defs/SourcePath/type",
																				keyword: "type",
																				params: { type: "string" },
																				message: "must be string"
																			};
																			s === null ? s = [e] : s.push(e), c++;
																		}
																	}
																	var p = o === c;
																	a ||= p;
																	let l = c;
																	if (n !== null) {
																		let e = {
																			instancePath: t + "/test_path",
																			schemaPath: "#/properties/test_path/anyOf/1/type",
																			keyword: "type",
																			params: { type: "null" },
																			message: "must be null"
																		};
																		s === null ? s = [e] : s.push(e), c++;
																	}
																	var p = l === c;
																	if (a ||= p, a) c = i, s !== null && (i ? s.length = i : s = null);
																	else {
																		let e = {
																			instancePath: t + "/test_path",
																			schemaPath: "#/properties/test_path/anyOf",
																			keyword: "anyOf",
																			params: {},
																			message: "must match a schema in anyOf"
																		};
																		return s === null ? s = [e] : s.push(e), c++, v.errors = s, !1;
																	}
																	var d = r === c;
																} else var d = !0;
															}
														}
													}
												}
											}
										}
									}
								}
							}
						}
					}
				}
			} else return v.errors = [{
				instancePath: t,
				schemaPath: "#/type",
				keyword: "type",
				params: { type: "object" },
				message: "must be object"
			}], !1;
		}
		return v.errors = s, c === 0;
	}
	v.evaluated = {
		props: !0,
		dynamicProps: !1,
		dynamicItems: !1
	};
	function y(e, { instancePath: t = "", parentData: a, parentDataProperty: o, rootData: s = e, dynamicAnchors: c = {} } = {}) {
		let l = y.evaluated;
		if (l.dynamicProps && (l.props = void 0), l.dynamicItems && (l.items = void 0), e && typeof e == "object" && !Array.isArray(e)) {
			let a;
			if (e.name === void 0 && (a = "name") || e.description === void 0 && (a = "description") || e.category === void 0 && (a = "category") || e.fields === void 0 && (a = "fields") || e.module_path === void 0 && (a = "module_path") || e.source_path === void 0 && (a = "source_path")) return y.errors = [{
				instancePath: t,
				schemaPath: "#/required",
				keyword: "required",
				params: { missingProperty: a },
				message: "must have required property '" + a + "'"
			}], !1;
			for (let n in e) if (n !== "name" && n !== "description" && n !== "display_name" && n !== "aliases" && n !== "category" && n !== "fields" && n !== "module_path" && n !== "source_path") return y.errors = [{
				instancePath: t,
				schemaPath: "#/additionalProperties",
				keyword: "additionalProperties",
				params: { additionalProperty: n },
				message: "must NOT have additional properties"
			}], !1;
			if (e.name !== void 0) {
				let n = e.name;
				if (typeof n == "string") {
					if (!r.test(n)) return y.errors = [{
						instancePath: t + "/name",
						schemaPath: "#/$defs/Name/pattern",
						keyword: "pattern",
						params: { pattern: "^[A-Za-z_][A-Za-z0-9_]*$" },
						message: "must match pattern \"^[A-Za-z_][A-Za-z0-9_]*$\""
					}], !1;
				} else return y.errors = [{
					instancePath: t + "/name",
					schemaPath: "#/$defs/Name/type",
					keyword: "type",
					params: { type: "string" },
					message: "must be string"
				}], !1;
				var u = !0;
			} else var u = !0;
			if (u) {
				if (e.description !== void 0) {
					if (typeof e.description != "string") return y.errors = [{
						instancePath: t + "/description",
						schemaPath: "#/properties/description/type",
						keyword: "type",
						params: { type: "string" },
						message: "must be string"
					}], !1;
					var u = !0;
				} else var u = !0;
				if (u) {
					if (e.display_name !== void 0) {
						let n = e.display_name;
						if (typeof n == "string") {
							if (i(n) < 1) return y.errors = [{
								instancePath: t + "/display_name",
								schemaPath: "#/properties/display_name/minLength",
								keyword: "minLength",
								params: { limit: 1 },
								message: "must NOT have fewer than 1 characters"
							}], !1;
						} else return y.errors = [{
							instancePath: t + "/display_name",
							schemaPath: "#/properties/display_name/type",
							keyword: "type",
							params: { type: "string" },
							message: "must be string"
						}], !1;
						var u = !0;
					} else var u = !0;
					if (u) {
						if (e.aliases !== void 0) {
							let n = e.aliases;
							if (Array.isArray(n)) {
								var d = !0;
								let e = n.length;
								for (let r = 0; r < e; r++) {
									let e = n[r];
									if (typeof e == "string") {
										if (i(e) < 1) return y.errors = [{
											instancePath: t + "/aliases/" + r,
											schemaPath: "#/properties/aliases/items/minLength",
											keyword: "minLength",
											params: { limit: 1 },
											message: "must NOT have fewer than 1 characters"
										}], !1;
									} else return y.errors = [{
										instancePath: t + "/aliases/" + r,
										schemaPath: "#/properties/aliases/items/type",
										keyword: "type",
										params: { type: "string" },
										message: "must be string"
									}], !1;
									var d = !0;
									if (!d) break;
								}
								if (d) {
									let e = n.length, r;
									if (e > 1) {
										let i = {};
										for (; e--;) {
											let a = n[e];
											if (typeof a == "string") {
												if (typeof i[a] == "number") return r = i[a], y.errors = [{
													instancePath: t + "/aliases",
													schemaPath: "#/properties/aliases/uniqueItems",
													keyword: "uniqueItems",
													params: {
														i: e,
														j: r
													},
													message: "must NOT have duplicate items (items ## " + r + " and " + e + " are identical)"
												}], !1;
												i[a] = e;
											}
										}
									}
								}
							} else return y.errors = [{
								instancePath: t + "/aliases",
								schemaPath: "#/properties/aliases/type",
								keyword: "type",
								params: { type: "array" },
								message: "must be array"
							}], !1;
							var u = !0;
						} else var u = !0;
						if (u) {
							if (e.category !== void 0) {
								let r = e.category;
								if (typeof r != "string") return y.errors = [{
									instancePath: t + "/category",
									schemaPath: "#/$defs/Category/type",
									keyword: "type",
									params: { type: "string" },
									message: "must be string"
								}], !1;
								if (r !== "graph" && r !== "formula" && r !== "set" && r !== "algebraic" && r !== "misc") return y.errors = [{
									instancePath: t + "/category",
									schemaPath: "#/$defs/Category/enum",
									keyword: "enum",
									params: { allowedValues: n.enum },
									message: "must be equal to one of the allowed values"
								}], !1;
								var u = !0;
							} else var u = !0;
							if (u) {
								if (e.fields !== void 0) {
									let n = e.fields;
									if (Array.isArray(n)) {
										let e = n.length;
										for (let r = 0; r < e; r++) {
											let e = n[r];
											if (e && typeof e == "object" && !Array.isArray(e)) {
												let n;
												if (e.name === void 0 && (n = "name") || e.type_name === void 0 && (n = "type_name") || e.description === void 0 && (n = "description")) return y.errors = [{
													instancePath: t + "/fields/" + r,
													schemaPath: "#/$defs/Field/required",
													keyword: "required",
													params: { missingProperty: n },
													message: "must have required property '" + n + "'"
												}], !1;
												for (let n in e) if (n !== "name" && n !== "type_name" && n !== "description") return y.errors = [{
													instancePath: t + "/fields/" + r,
													schemaPath: "#/$defs/Field/additionalProperties",
													keyword: "additionalProperties",
													params: { additionalProperty: n },
													message: "must NOT have additional properties"
												}], !1;
												if (e.name !== void 0) {
													let n = e.name;
													if (typeof n == "string") {
														if (i(n) < 1) return y.errors = [{
															instancePath: t + "/fields/" + r + "/name",
															schemaPath: "#/$defs/Field/properties/name/minLength",
															keyword: "minLength",
															params: { limit: 1 },
															message: "must NOT have fewer than 1 characters"
														}], !1;
													} else return y.errors = [{
														instancePath: t + "/fields/" + r + "/name",
														schemaPath: "#/$defs/Field/properties/name/type",
														keyword: "type",
														params: { type: "string" },
														message: "must be string"
													}], !1;
													var f = !0;
												} else var f = !0;
												if (f) {
													if (e.type_name !== void 0) {
														let n = e.type_name;
														if (typeof n == "string") {
															if (i(n) < 1) return y.errors = [{
																instancePath: t + "/fields/" + r + "/type_name",
																schemaPath: "#/$defs/Field/properties/type_name/minLength",
																keyword: "minLength",
																params: { limit: 1 },
																message: "must NOT have fewer than 1 characters"
															}], !1;
														} else return y.errors = [{
															instancePath: t + "/fields/" + r + "/type_name",
															schemaPath: "#/$defs/Field/properties/type_name/type",
															keyword: "type",
															params: { type: "string" },
															message: "must be string"
														}], !1;
														var f = !0;
													} else var f = !0;
													if (f) {
														if (e.description !== void 0) {
															if (typeof e.description != "string") return y.errors = [{
																instancePath: t + "/fields/" + r + "/description",
																schemaPath: "#/$defs/Field/properties/description/type",
																keyword: "type",
																params: { type: "string" },
																message: "must be string"
															}], !1;
															var f = !0;
														} else var f = !0;
													}
												}
											} else return y.errors = [{
												instancePath: t + "/fields/" + r,
												schemaPath: "#/$defs/Field/type",
												keyword: "type",
												params: { type: "object" },
												message: "must be object"
											}], !1;
										}
									} else return y.errors = [{
										instancePath: t + "/fields",
										schemaPath: "#/properties/fields/type",
										keyword: "type",
										params: { type: "array" },
										message: "must be array"
									}], !1;
									var u = !0;
								} else var u = !0;
								if (u) {
									if (e.module_path !== void 0) {
										let n = e.module_path;
										if (typeof n == "string") {
											if (i(n) < 1) return y.errors = [{
												instancePath: t + "/module_path",
												schemaPath: "#/properties/module_path/minLength",
												keyword: "minLength",
												params: { limit: 1 },
												message: "must NOT have fewer than 1 characters"
											}], !1;
										} else return y.errors = [{
											instancePath: t + "/module_path",
											schemaPath: "#/properties/module_path/type",
											keyword: "type",
											params: { type: "string" },
											message: "must be string"
										}], !1;
										var u = !0;
									} else var u = !0;
									if (u) {
										if (e.source_path !== void 0) {
											let n = e.source_path;
											if (typeof n == "string") {
												if (!_.test(n)) return y.errors = [{
													instancePath: t + "/source_path",
													schemaPath: "#/$defs/SourcePath/pattern",
													keyword: "pattern",
													params: { pattern: "^src/(?:[A-Za-z0-9_-]+/)*[A-Za-z0-9_-]+\\.rs$" },
													message: "must match pattern \"^src/(?:[A-Za-z0-9_-]+/)*[A-Za-z0-9_-]+\\.rs$\""
												}], !1;
											} else return y.errors = [{
												instancePath: t + "/source_path",
												schemaPath: "#/$defs/SourcePath/type",
												keyword: "type",
												params: { type: "string" },
												message: "must be string"
											}], !1;
											var u = !0;
										} else var u = !0;
									}
								}
							}
						}
					}
				}
			}
		} else return y.errors = [{
			instancePath: t,
			schemaPath: "#/type",
			keyword: "type",
			params: { type: "object" },
			message: "must be object"
		}], !1;
		return y.errors = null, !0;
	}
	y.evaluated = {
		props: !0,
		dynamicProps: !1,
		dynamicItems: !1
	};
	function b(e, { instancePath: t = "", parentData: n, parentDataProperty: r, rootData: i = e, dynamicAnchors: a = {} } = {}) {
		let o = null, s = 0, c = b.evaluated;
		if (c.dynamicProps && (c.props = void 0), c.dynamicItems && (c.items = void 0), s === 0) {
			if (e && typeof e == "object" && !Array.isArray(e)) {
				let n;
				if (e.schema_version === void 0 && (n = "schema_version") || e.kind === void 0 && (n = "kind") || e.nodes === void 0 && (n = "nodes") || e.edges === void 0 && (n = "edges") || e.schemas === void 0 && (n = "schemas")) return b.errors = [{
					instancePath: t,
					schemaPath: "#/required",
					keyword: "required",
					params: { missingProperty: n },
					message: "must have required property '" + n + "'"
				}], !1;
				{
					let n = s;
					for (let n in e) if (n !== "schema_version" && n !== "kind" && n !== "nodes" && n !== "edges" && n !== "schemas") return b.errors = [{
						instancePath: t,
						schemaPath: "#/additionalProperties",
						keyword: "additionalProperties",
						params: { additionalProperty: n },
						message: "must NOT have additional properties"
					}], !1;
					if (n === s) {
						if (e.schema_version !== void 0) {
							let n = e.schema_version, r = s;
							if (typeof n != "number" || n % 1 || isNaN(n) || !isFinite(n)) return b.errors = [{
								instancePath: t + "/schema_version",
								schemaPath: "#/properties/schema_version/type",
								keyword: "type",
								params: { type: "integer" },
								message: "must be integer"
							}], !1;
							if (n !== 1) return b.errors = [{
								instancePath: t + "/schema_version",
								schemaPath: "#/properties/schema_version/const",
								keyword: "const",
								params: { allowedValue: 1 },
								message: "must be equal to constant"
							}], !1;
							var l = r === s;
						} else var l = !0;
						if (l) {
							if (e.kind !== void 0) {
								let n = e.kind, r = s;
								if (typeof n != "string") return b.errors = [{
									instancePath: t + "/kind",
									schemaPath: "#/properties/kind/type",
									keyword: "type",
									params: { type: "string" },
									message: "must be string"
								}], !1;
								if (n !== "record") return b.errors = [{
									instancePath: t + "/kind",
									schemaPath: "#/properties/kind/const",
									keyword: "const",
									params: { allowedValue: "record" },
									message: "must be equal to constant"
								}], !1;
								var l = r === s;
							} else var l = !0;
							if (l) {
								if (e.nodes !== void 0) {
									let n = e.nodes, r = s;
									if (s === r) {
										if (Array.isArray(n)) {
											if (n.length < 1) return b.errors = [{
												instancePath: t + "/nodes",
												schemaPath: "#/properties/nodes/minItems",
												keyword: "minItems",
												params: { limit: 1 },
												message: "must NOT have fewer than 1 items"
											}], !1;
											{
												let e = n.length;
												for (let r = 0; r < e; r++) {
													let e = s;
													if (p(n[r], {
														instancePath: t + "/nodes/" + r,
														parentData: n,
														parentDataProperty: r,
														rootData: i,
														dynamicAnchors: a
													}) || (o = o === null ? p.errors : o.concat(p.errors), s = o.length), e !== s) break;
												}
											}
										} else return b.errors = [{
											instancePath: t + "/nodes",
											schemaPath: "#/properties/nodes/type",
											keyword: "type",
											params: { type: "array" },
											message: "must be array"
										}], !1;
									}
									var l = r === s;
								} else var l = !0;
								if (l) {
									if (e.edges !== void 0) {
										let n = e.edges, r = s;
										if (s === r) {
											if (Array.isArray(n)) {
												let e = n.length;
												for (let r = 0; r < e; r++) {
													let e = s;
													if (v(n[r], {
														instancePath: t + "/edges/" + r,
														parentData: n,
														parentDataProperty: r,
														rootData: i,
														dynamicAnchors: a
													}) || (o = o === null ? v.errors : o.concat(v.errors), s = o.length), e !== s) break;
												}
											} else return b.errors = [{
												instancePath: t + "/edges",
												schemaPath: "#/properties/edges/type",
												keyword: "type",
												params: { type: "array" },
												message: "must be array"
											}], !1;
										}
										var l = r === s;
									} else var l = !0;
									if (l) {
										if (e.schemas !== void 0) {
											let n = e.schemas, r = s;
											if (s === r) {
												if (Array.isArray(n)) {
													if (n.length > 1) return b.errors = [{
														instancePath: t + "/schemas",
														schemaPath: "#/properties/schemas/maxItems",
														keyword: "maxItems",
														params: { limit: 1 },
														message: "must NOT have more than 1 items"
													}], !1;
													if (n.length < 1) return b.errors = [{
														instancePath: t + "/schemas",
														schemaPath: "#/properties/schemas/minItems",
														keyword: "minItems",
														params: { limit: 1 },
														message: "must NOT have fewer than 1 items"
													}], !1;
													{
														let e = n.length;
														for (let r = 0; r < e; r++) {
															let e = s;
															if (y(n[r], {
																instancePath: t + "/schemas/" + r,
																parentData: n,
																parentDataProperty: r,
																rootData: i,
																dynamicAnchors: a
															}) || (o = o === null ? y.errors : o.concat(y.errors), s = o.length), e !== s) break;
														}
													}
												} else return b.errors = [{
													instancePath: t + "/schemas",
													schemaPath: "#/properties/schemas/type",
													keyword: "type",
													params: { type: "array" },
													message: "must be array"
												}], !1;
											}
											var l = r === s;
										} else var l = !0;
									}
								}
							}
						}
					}
				}
			} else return b.errors = [{
				instancePath: t,
				schemaPath: "#/type",
				keyword: "type",
				params: { type: "object" },
				message: "must be object"
			}], !1;
		}
		return b.errors = o, s === 0;
	}
	b.evaluated = {
		props: !0,
		dynamicProps: !1,
		dynamicItems: !1
	};
	function x(e, { instancePath: t = "", parentData: n, parentDataProperty: r, rootData: i = e, dynamicAnchors: a = {} } = {}) {
		let o = null, s = 0, l = x.evaluated;
		l.dynamicProps && (l.props = void 0), l.dynamicItems && (l.items = void 0);
		let u = s, d = !1, f = null, p = s;
		c(e, {
			instancePath: t,
			parentData: n,
			parentDataProperty: r,
			rootData: i,
			dynamicAnchors: a
		}) || (o = o === null ? c.errors : o.concat(c.errors), s = o.length);
		var m = p === s;
		if (m) {
			d = !0, f = 0;
			var h = !0;
		}
		let g = s;
		b(e, {
			instancePath: t,
			parentData: n,
			parentDataProperty: r,
			rootData: i,
			dynamicAnchors: a
		}) || (o = o === null ? b.errors : o.concat(b.errors), s = o.length);
		var m = g === s;
		if (m && d ? (d = !1, f = [f, 1]) : m && (d = !0, f = 1, h !== !0 && (h = !0)), d) s = u, o !== null && (u ? o.length = u : o = null);
		else {
			let e = {
				instancePath: t,
				schemaPath: "#/oneOf",
				keyword: "oneOf",
				params: { passingSchemas: f },
				message: "must match exactly one schema in oneOf"
			};
			return o === null ? o = [e] : o.push(e), s++, x.errors = o, !1;
		}
		return x.errors = o, l.props = h, s === 0;
	}
	x.evaluated = {
		dynamicProps: !0,
		dynamicItems: !1
	};
})))().validateAtlas;
export { n as validateAtlas };
