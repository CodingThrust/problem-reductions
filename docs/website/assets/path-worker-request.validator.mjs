//#endregion
var e = (/* @__PURE__ */ ((e, t) => () => (t || (e((t = { exports: {} }).exports, t, t.exports), e = null), t.exports))(((e) => {
	e.validatePathRequestEnvelope = t;
	function t(e, { instancePath: n = "", parentData: r, parentDataProperty: i, rootData: a = e, dynamicAnchors: o = {} } = {}) {
		let s = null, c = 0, l = t.evaluated;
		l.dynamicProps && (l.props = void 0), l.dynamicItems && (l.items = void 0);
		let u = c, d = !1, f = null, p = c;
		if (c === p) {
			if (e && typeof e == "object" && !Array.isArray(e)) {
				let t;
				if (e.schema_version === void 0 && (t = "schema_version") || e.kind === void 0 && (t = "kind") || e.source === void 0 && (t = "source") || e.target === void 0 && (t = "target") || e.limit === void 0 && (t = "limit")) {
					let e = {
						instancePath: n,
						schemaPath: "#/oneOf/0/required",
						keyword: "required",
						params: { missingProperty: t },
						message: "must have required property '" + t + "'"
					};
					s === null ? s = [e] : s.push(e), c++;
				} else {
					let t = c;
					for (let t in e) if (t !== "schema_version" && t !== "kind" && t !== "source" && t !== "target" && t !== "limit") {
						let e = {
							instancePath: n,
							schemaPath: "#/oneOf/0/additionalProperties",
							keyword: "additionalProperties",
							params: { additionalProperty: t },
							message: "must NOT have additional properties"
						};
						s === null ? s = [e] : s.push(e), c++;
						break;
					}
					if (t === c) {
						if (e.schema_version !== void 0) {
							let t = e.schema_version, r = c;
							if (typeof t != "number" || t % 1 || isNaN(t) || !isFinite(t)) {
								let e = {
									instancePath: n + "/schema_version",
									schemaPath: "#/oneOf/0/properties/schema_version/type",
									keyword: "type",
									params: { type: "integer" },
									message: "must be integer"
								};
								s === null ? s = [e] : s.push(e), c++;
							}
							if (t !== 1) {
								let e = {
									instancePath: n + "/schema_version",
									schemaPath: "#/oneOf/0/properties/schema_version/const",
									keyword: "const",
									params: { allowedValue: 1 },
									message: "must be equal to constant"
								};
								s === null ? s = [e] : s.push(e), c++;
							}
							var m = r === c;
						} else var m = !0;
						if (m) {
							if (e.kind !== void 0) {
								let t = e.kind, r = c;
								if (typeof t != "string") {
									let e = {
										instancePath: n + "/kind",
										schemaPath: "#/oneOf/0/properties/kind/type",
										keyword: "type",
										params: { type: "string" },
										message: "must be string"
									};
									s === null ? s = [e] : s.push(e), c++;
								}
								if (t !== "search") {
									let e = {
										instancePath: n + "/kind",
										schemaPath: "#/oneOf/0/properties/kind/const",
										keyword: "const",
										params: { allowedValue: "search" },
										message: "must be equal to constant"
									};
									s === null ? s = [e] : s.push(e), c++;
								}
								var m = r === c;
							} else var m = !0;
							if (m) {
								if (e.source !== void 0) {
									let t = e.source, r = c;
									if (typeof t != "number" || t % 1 || isNaN(t) || !isFinite(t)) {
										let e = {
											instancePath: n + "/source",
											schemaPath: "#/oneOf/0/properties/source/type",
											keyword: "type",
											params: { type: "integer" },
											message: "must be integer"
										};
										s === null ? s = [e] : s.push(e), c++;
									}
									if (c === r && typeof t == "number" && isFinite(t)) {
										if (t > 9007199254740991 || isNaN(t)) {
											let e = {
												instancePath: n + "/source",
												schemaPath: "#/oneOf/0/properties/source/maximum",
												keyword: "maximum",
												params: {
													comparison: "<=",
													limit: 9007199254740991
												},
												message: "must be <= 9007199254740991"
											};
											s === null ? s = [e] : s.push(e), c++;
										} else if (t < 0 || isNaN(t)) {
											let e = {
												instancePath: n + "/source",
												schemaPath: "#/oneOf/0/properties/source/minimum",
												keyword: "minimum",
												params: {
													comparison: ">=",
													limit: 0
												},
												message: "must be >= 0"
											};
											s === null ? s = [e] : s.push(e), c++;
										}
									}
									var m = r === c;
								} else var m = !0;
								if (m) {
									if (e.target !== void 0) {
										let t = e.target, r = c;
										if (typeof t != "number" || t % 1 || isNaN(t) || !isFinite(t)) {
											let e = {
												instancePath: n + "/target",
												schemaPath: "#/oneOf/0/properties/target/type",
												keyword: "type",
												params: { type: "integer" },
												message: "must be integer"
											};
											s === null ? s = [e] : s.push(e), c++;
										}
										if (c === r && typeof t == "number" && isFinite(t)) {
											if (t > 9007199254740991 || isNaN(t)) {
												let e = {
													instancePath: n + "/target",
													schemaPath: "#/oneOf/0/properties/target/maximum",
													keyword: "maximum",
													params: {
														comparison: "<=",
														limit: 9007199254740991
													},
													message: "must be <= 9007199254740991"
												};
												s === null ? s = [e] : s.push(e), c++;
											} else if (t < 0 || isNaN(t)) {
												let e = {
													instancePath: n + "/target",
													schemaPath: "#/oneOf/0/properties/target/minimum",
													keyword: "minimum",
													params: {
														comparison: ">=",
														limit: 0
													},
													message: "must be >= 0"
												};
												s === null ? s = [e] : s.push(e), c++;
											}
										}
										var m = r === c;
									} else var m = !0;
									if (m) {
										if (e.limit !== void 0) {
											let t = e.limit, r = c;
											if (typeof t != "number" || t % 1 || isNaN(t) || !isFinite(t)) {
												let e = {
													instancePath: n + "/limit",
													schemaPath: "#/oneOf/0/properties/limit/type",
													keyword: "type",
													params: { type: "integer" },
													message: "must be integer"
												};
												s === null ? s = [e] : s.push(e), c++;
											}
											if (c === r && typeof t == "number" && isFinite(t)) {
												if (t > 999 || isNaN(t)) {
													let e = {
														instancePath: n + "/limit",
														schemaPath: "#/oneOf/0/properties/limit/maximum",
														keyword: "maximum",
														params: {
															comparison: "<=",
															limit: 999
														},
														message: "must be <= 999"
													};
													s === null ? s = [e] : s.push(e), c++;
												} else if (t < 1 || isNaN(t)) {
													let e = {
														instancePath: n + "/limit",
														schemaPath: "#/oneOf/0/properties/limit/minimum",
														keyword: "minimum",
														params: {
															comparison: ">=",
															limit: 1
														},
														message: "must be >= 1"
													};
													s === null ? s = [e] : s.push(e), c++;
												}
											}
											var m = r === c;
										} else var m = !0;
									}
								}
							}
						}
					}
				}
			} else {
				let e = {
					instancePath: n,
					schemaPath: "#/oneOf/0/type",
					keyword: "type",
					params: { type: "object" },
					message: "must be object"
				};
				s === null ? s = [e] : s.push(e), c++;
			}
		}
		var h = p === c;
		if (h) {
			d = !0, f = 0;
			var g = !0;
		}
		let _ = c;
		if (c === _) {
			if (e && typeof e == "object" && !Array.isArray(e)) {
				let t;
				if (e.schema_version === void 0 && (t = "schema_version") || e.kind === void 0 && (t = "kind") || e.source === void 0 && (t = "source") || e.target === void 0 && (t = "target") || e.limit === void 0 && (t = "limit") || e.cached === void 0 && (t = "cached")) {
					let e = {
						instancePath: n,
						schemaPath: "#/oneOf/1/required",
						keyword: "required",
						params: { missingProperty: t },
						message: "must have required property '" + t + "'"
					};
					s === null ? s = [e] : s.push(e), c++;
				} else {
					let t = c;
					for (let t in e) if (t !== "schema_version" && t !== "kind" && t !== "source" && t !== "target" && t !== "limit" && t !== "cached") {
						let e = {
							instancePath: n,
							schemaPath: "#/oneOf/1/additionalProperties",
							keyword: "additionalProperties",
							params: { additionalProperty: t },
							message: "must NOT have additional properties"
						};
						s === null ? s = [e] : s.push(e), c++;
						break;
					}
					if (t === c) {
						if (e.schema_version !== void 0) {
							let t = e.schema_version, r = c;
							if (typeof t != "number" || t % 1 || isNaN(t) || !isFinite(t)) {
								let e = {
									instancePath: n + "/schema_version",
									schemaPath: "#/oneOf/1/properties/schema_version/type",
									keyword: "type",
									params: { type: "integer" },
									message: "must be integer"
								};
								s === null ? s = [e] : s.push(e), c++;
							}
							if (t !== 1) {
								let e = {
									instancePath: n + "/schema_version",
									schemaPath: "#/oneOf/1/properties/schema_version/const",
									keyword: "const",
									params: { allowedValue: 1 },
									message: "must be equal to constant"
								};
								s === null ? s = [e] : s.push(e), c++;
							}
							var v = r === c;
						} else var v = !0;
						if (v) {
							if (e.kind !== void 0) {
								let t = e.kind, r = c;
								if (typeof t != "string") {
									let e = {
										instancePath: n + "/kind",
										schemaPath: "#/oneOf/1/properties/kind/type",
										keyword: "type",
										params: { type: "string" },
										message: "must be string"
									};
									s === null ? s = [e] : s.push(e), c++;
								}
								if (t !== "compare") {
									let e = {
										instancePath: n + "/kind",
										schemaPath: "#/oneOf/1/properties/kind/const",
										keyword: "const",
										params: { allowedValue: "compare" },
										message: "must be equal to constant"
									};
									s === null ? s = [e] : s.push(e), c++;
								}
								var v = r === c;
							} else var v = !0;
							if (v) {
								if (e.source !== void 0) {
									let t = e.source, r = c;
									if (typeof t != "number" || t % 1 || isNaN(t) || !isFinite(t)) {
										let e = {
											instancePath: n + "/source",
											schemaPath: "#/oneOf/1/properties/source/type",
											keyword: "type",
											params: { type: "integer" },
											message: "must be integer"
										};
										s === null ? s = [e] : s.push(e), c++;
									}
									if (c === r && typeof t == "number" && isFinite(t)) {
										if (t > 9007199254740991 || isNaN(t)) {
											let e = {
												instancePath: n + "/source",
												schemaPath: "#/oneOf/1/properties/source/maximum",
												keyword: "maximum",
												params: {
													comparison: "<=",
													limit: 9007199254740991
												},
												message: "must be <= 9007199254740991"
											};
											s === null ? s = [e] : s.push(e), c++;
										} else if (t < 0 || isNaN(t)) {
											let e = {
												instancePath: n + "/source",
												schemaPath: "#/oneOf/1/properties/source/minimum",
												keyword: "minimum",
												params: {
													comparison: ">=",
													limit: 0
												},
												message: "must be >= 0"
											};
											s === null ? s = [e] : s.push(e), c++;
										}
									}
									var v = r === c;
								} else var v = !0;
								if (v) {
									if (e.target !== void 0) {
										let t = e.target, r = c;
										if (typeof t != "number" || t % 1 || isNaN(t) || !isFinite(t)) {
											let e = {
												instancePath: n + "/target",
												schemaPath: "#/oneOf/1/properties/target/type",
												keyword: "type",
												params: { type: "integer" },
												message: "must be integer"
											};
											s === null ? s = [e] : s.push(e), c++;
										}
										if (c === r && typeof t == "number" && isFinite(t)) {
											if (t > 9007199254740991 || isNaN(t)) {
												let e = {
													instancePath: n + "/target",
													schemaPath: "#/oneOf/1/properties/target/maximum",
													keyword: "maximum",
													params: {
														comparison: "<=",
														limit: 9007199254740991
													},
													message: "must be <= 9007199254740991"
												};
												s === null ? s = [e] : s.push(e), c++;
											} else if (t < 0 || isNaN(t)) {
												let e = {
													instancePath: n + "/target",
													schemaPath: "#/oneOf/1/properties/target/minimum",
													keyword: "minimum",
													params: {
														comparison: ">=",
														limit: 0
													},
													message: "must be >= 0"
												};
												s === null ? s = [e] : s.push(e), c++;
											}
										}
										var v = r === c;
									} else var v = !0;
									if (v) {
										if (e.limit !== void 0) {
											let t = e.limit, r = c;
											if (typeof t != "number" || t % 1 || isNaN(t) || !isFinite(t)) {
												let e = {
													instancePath: n + "/limit",
													schemaPath: "#/oneOf/1/properties/limit/type",
													keyword: "type",
													params: { type: "integer" },
													message: "must be integer"
												};
												s === null ? s = [e] : s.push(e), c++;
											}
											if (c === r && typeof t == "number" && isFinite(t)) {
												if (t > 999 || isNaN(t)) {
													let e = {
														instancePath: n + "/limit",
														schemaPath: "#/oneOf/1/properties/limit/maximum",
														keyword: "maximum",
														params: {
															comparison: "<=",
															limit: 999
														},
														message: "must be <= 999"
													};
													s === null ? s = [e] : s.push(e), c++;
												} else if (t < 1 || isNaN(t)) {
													let e = {
														instancePath: n + "/limit",
														schemaPath: "#/oneOf/1/properties/limit/minimum",
														keyword: "minimum",
														params: {
															comparison: ">=",
															limit: 1
														},
														message: "must be >= 1"
													};
													s === null ? s = [e] : s.push(e), c++;
												}
											}
											var v = r === c;
										} else var v = !0;
									}
								}
							}
						}
					}
				}
			} else {
				let e = {
					instancePath: n,
					schemaPath: "#/oneOf/1/type",
					keyword: "type",
					params: { type: "object" },
					message: "must be object"
				};
				s === null ? s = [e] : s.push(e), c++;
			}
		}
		var h = _ === c;
		if (h && d ? (d = !1, f = [f, 1]) : h && (d = !0, f = 1, g !== !0 && (g = !0)), d) c = u, s !== null && (u ? s.length = u : s = null);
		else {
			let e = {
				instancePath: n,
				schemaPath: "#/oneOf",
				keyword: "oneOf",
				params: { passingSchemas: f },
				message: "must match exactly one schema in oneOf"
			};
			return s === null ? s = [e] : s.push(e), c++, t.errors = s, !1;
		}
		return t.errors = s, l.props = g, c === 0;
	}
	t.evaluated = {
		dynamicProps: !0,
		dynamicItems: !1
	};
})))().validatePathRequestEnvelope;
export { e as validatePathRequestEnvelope };
