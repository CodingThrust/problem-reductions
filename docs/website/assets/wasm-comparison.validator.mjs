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
	e.validateWasmComparison = l;
	var n = {
		type: "object",
		properties: {
			dominates: { $ref: "#/$defs/Dominance" },
			paths: {
				type: "array",
				items: {
					type: "integer",
					maximum: 9007199254740991,
					minimum: 1
				},
				maxItems: 2,
				minItems: 2
			},
			verdict: {
				type: "string",
				enum: [
					"Dominated",
					"Equivalent",
					"Incomparable",
					"Unknown"
				]
			}
		},
		additionalProperties: !1,
		required: ["paths", "verdict"]
	};
	function r(e, { instancePath: t = "", parentData: i, parentDataProperty: a, rootData: o = e, dynamicAnchors: s = {} } = {}) {
		let c = r.evaluated;
		if (c.dynamicProps && (c.props = void 0), c.dynamicItems && (c.items = void 0), e && typeof e == "object" && !Array.isArray(e)) {
			let i;
			if (e.paths === void 0 && (i = "paths") || e.verdict === void 0 && (i = "verdict")) return r.errors = [{
				instancePath: t,
				schemaPath: "#/required",
				keyword: "required",
				params: { missingProperty: i },
				message: "must have required property '" + i + "'"
			}], !1;
			for (let n in e) if (n !== "dominates" && n !== "paths" && n !== "verdict") return r.errors = [{
				instancePath: t,
				schemaPath: "#/additionalProperties",
				keyword: "additionalProperties",
				params: { additionalProperty: n },
				message: "must NOT have additional properties"
			}], !1;
			if (e.dominates !== void 0) {
				let n = e.dominates;
				if (n && typeof n == "object" && !Array.isArray(n)) {
					let e;
					if (n.winner === void 0 && (e = "winner") || n.loser === void 0 && (e = "loser")) return r.errors = [{
						instancePath: t + "/dominates",
						schemaPath: "#/$defs/Dominance/required",
						keyword: "required",
						params: { missingProperty: e },
						message: "must have required property '" + e + "'"
					}], !1;
					for (let e in n) if (e !== "loser" && e !== "winner") return r.errors = [{
						instancePath: t + "/dominates",
						schemaPath: "#/$defs/Dominance/additionalProperties",
						keyword: "additionalProperties",
						params: { additionalProperty: e },
						message: "must NOT have additional properties"
					}], !1;
					if (n.loser !== void 0) {
						let e = n.loser;
						if (typeof e != "number" || e % 1 || isNaN(e) || !isFinite(e)) return r.errors = [{
							instancePath: t + "/dominates/loser",
							schemaPath: "#/$defs/Dominance/properties/loser/type",
							keyword: "type",
							params: { type: "integer" },
							message: "must be integer"
						}], !1;
						if (typeof e == "number" && isFinite(e)) {
							if (e > 9007199254740991 || isNaN(e)) return r.errors = [{
								instancePath: t + "/dominates/loser",
								schemaPath: "#/$defs/Dominance/properties/loser/maximum",
								keyword: "maximum",
								params: {
									comparison: "<=",
									limit: 9007199254740991
								},
								message: "must be <= 9007199254740991"
							}], !1;
							if (e < 1 || isNaN(e)) return r.errors = [{
								instancePath: t + "/dominates/loser",
								schemaPath: "#/$defs/Dominance/properties/loser/minimum",
								keyword: "minimum",
								params: {
									comparison: ">=",
									limit: 1
								},
								message: "must be >= 1"
							}], !1;
						}
						var l = !0;
					} else var l = !0;
					if (l) {
						if (n.winner !== void 0) {
							let e = n.winner;
							if (typeof e != "number" || e % 1 || isNaN(e) || !isFinite(e)) return r.errors = [{
								instancePath: t + "/dominates/winner",
								schemaPath: "#/$defs/Dominance/properties/winner/type",
								keyword: "type",
								params: { type: "integer" },
								message: "must be integer"
							}], !1;
							if (typeof e == "number" && isFinite(e)) {
								if (e > 9007199254740991 || isNaN(e)) return r.errors = [{
									instancePath: t + "/dominates/winner",
									schemaPath: "#/$defs/Dominance/properties/winner/maximum",
									keyword: "maximum",
									params: {
										comparison: "<=",
										limit: 9007199254740991
									},
									message: "must be <= 9007199254740991"
								}], !1;
								if (e < 1 || isNaN(e)) return r.errors = [{
									instancePath: t + "/dominates/winner",
									schemaPath: "#/$defs/Dominance/properties/winner/minimum",
									keyword: "minimum",
									params: {
										comparison: ">=",
										limit: 1
									},
									message: "must be >= 1"
								}], !1;
							}
							var l = !0;
						} else var l = !0;
					}
				} else return r.errors = [{
					instancePath: t + "/dominates",
					schemaPath: "#/$defs/Dominance/type",
					keyword: "type",
					params: { type: "object" },
					message: "must be object"
				}], !1;
				var u = !0;
			} else var u = !0;
			if (u) {
				if (e.paths !== void 0) {
					let n = e.paths;
					if (Array.isArray(n)) {
						if (n.length > 2) return r.errors = [{
							instancePath: t + "/paths",
							schemaPath: "#/properties/paths/maxItems",
							keyword: "maxItems",
							params: { limit: 2 },
							message: "must NOT have more than 2 items"
						}], !1;
						if (n.length < 2) return r.errors = [{
							instancePath: t + "/paths",
							schemaPath: "#/properties/paths/minItems",
							keyword: "minItems",
							params: { limit: 2 },
							message: "must NOT have fewer than 2 items"
						}], !1;
						{
							let e = n.length;
							for (let i = 0; i < e; i++) {
								let e = n[i];
								if (typeof e != "number" || e % 1 || isNaN(e) || !isFinite(e)) return r.errors = [{
									instancePath: t + "/paths/" + i,
									schemaPath: "#/properties/paths/items/type",
									keyword: "type",
									params: { type: "integer" },
									message: "must be integer"
								}], !1;
								if (typeof e == "number" && isFinite(e)) {
									if (e > 9007199254740991 || isNaN(e)) return r.errors = [{
										instancePath: t + "/paths/" + i,
										schemaPath: "#/properties/paths/items/maximum",
										keyword: "maximum",
										params: {
											comparison: "<=",
											limit: 9007199254740991
										},
										message: "must be <= 9007199254740991"
									}], !1;
									if (e < 1 || isNaN(e)) return r.errors = [{
										instancePath: t + "/paths/" + i,
										schemaPath: "#/properties/paths/items/minimum",
										keyword: "minimum",
										params: {
											comparison: ">=",
											limit: 1
										},
										message: "must be >= 1"
									}], !1;
								}
							}
						}
					} else return r.errors = [{
						instancePath: t + "/paths",
						schemaPath: "#/properties/paths/type",
						keyword: "type",
						params: { type: "array" },
						message: "must be array"
					}], !1;
					var u = !0;
				} else var u = !0;
				if (u) {
					if (e.verdict !== void 0) {
						let i = e.verdict;
						if (typeof i != "string") return r.errors = [{
							instancePath: t + "/verdict",
							schemaPath: "#/properties/verdict/type",
							keyword: "type",
							params: { type: "string" },
							message: "must be string"
						}], !1;
						if (i !== "Dominated" && i !== "Equivalent" && i !== "Incomparable" && i !== "Unknown") return r.errors = [{
							instancePath: t + "/verdict",
							schemaPath: "#/properties/verdict/enum",
							keyword: "enum",
							params: { allowedValues: n.properties.verdict.enum },
							message: "must be equal to one of the allowed values"
						}], !1;
						var u = !0;
					} else var u = !0;
				}
			}
		} else return r.errors = [{
			instancePath: t,
			schemaPath: "#/type",
			keyword: "type",
			params: { type: "object" },
			message: "must be object"
		}], !1;
		return r.errors = null, !0;
	}
	r.evaluated = {
		props: !0,
		dynamicProps: !1,
		dynamicItems: !1
	};
	var i = t().default, a = {
		type: "object",
		properties: {
			accuracy: { $ref: "#/$defs/ComparisonAccuracy" },
			certificate: {
				type: ["string", "null"],
				minLength: 1
			}
		},
		additionalProperties: !1,
		required: ["certificate", "accuracy"]
	};
	function o(e, { instancePath: t = "", parentData: n, parentDataProperty: r, rootData: i = e, dynamicAnchors: a = {} } = {}) {
		let s = null, c = 0, l = o.evaluated;
		l.dynamicProps && (l.props = void 0), l.dynamicItems && (l.items = void 0);
		let u = c, d = !1, f = null, p = c;
		if (c === p) {
			if (e && typeof e == "object" && !Array.isArray(e)) {
				let n;
				if (e.kind === void 0 && (n = "kind")) {
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
					for (let n in e) if (n !== "kind") {
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
					if (n === c && e.kind !== void 0) {
						let n = e.kind;
						if (typeof n != "string") {
							let e = {
								instancePath: t + "/kind",
								schemaPath: "#/oneOf/0/properties/kind/type",
								keyword: "type",
								params: { type: "string" },
								message: "must be string"
							};
							s === null ? s = [e] : s.push(e), c++;
						}
						if (n !== "exact") {
							let e = {
								instancePath: t + "/kind",
								schemaPath: "#/oneOf/0/properties/kind/const",
								keyword: "const",
								params: { allowedValue: "exact" },
								message: "must be equal to constant"
							};
							s === null ? s = [e] : s.push(e), c++;
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
		var m = p === c;
		if (m) {
			d = !0, f = 0;
			var h = !0;
		}
		let g = c;
		if (c === g) {
			if (e && typeof e == "object" && !Array.isArray(e)) {
				let n;
				if (e.kind === void 0 && (n = "kind")) {
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
					for (let n in e) if (n !== "kind") {
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
					if (n === c && e.kind !== void 0) {
						let n = e.kind;
						if (typeof n != "string") {
							let e = {
								instancePath: t + "/kind",
								schemaPath: "#/oneOf/1/properties/kind/type",
								keyword: "type",
								params: { type: "string" },
								message: "must be string"
							};
							s === null ? s = [e] : s.push(e), c++;
						}
						if (n !== "upper_bound") {
							let e = {
								instancePath: t + "/kind",
								schemaPath: "#/oneOf/1/properties/kind/const",
								keyword: "const",
								params: { allowedValue: "upper_bound" },
								message: "must be equal to constant"
							};
							s === null ? s = [e] : s.push(e), c++;
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
		var m = g === c;
		if (m && d) d = !1, f = [f, 1];
		else {
			m && (d = !0, f = 1, h !== !0 && (h = !0));
			let n = c;
			if (c === n) {
				if (e && typeof e == "object" && !Array.isArray(e)) {
					let n;
					if (e.kind === void 0 && (n = "kind") || e.reason === void 0 && (n = "reason")) {
						let e = {
							instancePath: t,
							schemaPath: "#/oneOf/2/required",
							keyword: "required",
							params: { missingProperty: n },
							message: "must have required property '" + n + "'"
						};
						s === null ? s = [e] : s.push(e), c++;
					} else {
						let n = c;
						for (let n in e) if (n !== "kind" && n !== "reason") {
							let e = {
								instancePath: t,
								schemaPath: "#/oneOf/2/additionalProperties",
								keyword: "additionalProperties",
								params: { additionalProperty: n },
								message: "must NOT have additional properties"
							};
							s === null ? s = [e] : s.push(e), c++;
							break;
						}
						if (n === c) {
							if (e.kind !== void 0) {
								let n = e.kind, r = c;
								if (typeof n != "string") {
									let e = {
										instancePath: t + "/kind",
										schemaPath: "#/oneOf/2/properties/kind/type",
										keyword: "type",
										params: { type: "string" },
										message: "must be string"
									};
									s === null ? s = [e] : s.push(e), c++;
								}
								if (n !== "unavailable") {
									let e = {
										instancePath: t + "/kind",
										schemaPath: "#/oneOf/2/properties/kind/const",
										keyword: "const",
										params: { allowedValue: "unavailable" },
										message: "must be equal to constant"
									};
									s === null ? s = [e] : s.push(e), c++;
								}
								var _ = r === c;
							} else var _ = !0;
							if (_) {
								if (e.reason !== void 0) {
									let n = e.reason, r = c, i = c, a = !1, o = null, l = c;
									if (c === l) {
										if (n && typeof n == "object" && !Array.isArray(n)) {
											let e;
											if (n.kind === void 0 && (e = "kind") || n.detail === void 0 && (e = "detail")) {
												let n = {
													instancePath: t + "/reason",
													schemaPath: "#/$defs/ComparisonReason/oneOf/0/required",
													keyword: "required",
													params: { missingProperty: e },
													message: "must have required property '" + e + "'"
												};
												s === null ? s = [n] : s.push(n), c++;
											} else {
												let e = c;
												for (let e in n) if (e !== "detail" && e !== "kind") {
													let n = {
														instancePath: t + "/reason",
														schemaPath: "#/$defs/ComparisonReason/oneOf/0/additionalProperties",
														keyword: "additionalProperties",
														params: { additionalProperty: e },
														message: "must NOT have additional properties"
													};
													s === null ? s = [n] : s.push(n), c++;
													break;
												}
												if (e === c) {
													if (n.detail !== void 0) {
														let e = c;
														if (typeof n.detail != "string") {
															let e = {
																instancePath: t + "/reason/detail",
																schemaPath: "#/$defs/ComparisonReason/oneOf/0/properties/detail/type",
																keyword: "type",
																params: { type: "string" },
																message: "must be string"
															};
															s === null ? s = [e] : s.push(e), c++;
														}
														var v = e === c;
													} else var v = !0;
													if (v) {
														if (n.kind !== void 0) {
															let e = n.kind, r = c;
															if (typeof e != "string") {
																let e = {
																	instancePath: t + "/reason/kind",
																	schemaPath: "#/$defs/ComparisonReason/oneOf/0/properties/kind/type",
																	keyword: "type",
																	params: { type: "string" },
																	message: "must be string"
																};
																s === null ? s = [e] : s.push(e), c++;
															}
															if (e !== "not_polynomial") {
																let e = {
																	instancePath: t + "/reason/kind",
																	schemaPath: "#/$defs/ComparisonReason/oneOf/0/properties/kind/const",
																	keyword: "const",
																	params: { allowedValue: "not_polynomial" },
																	message: "must be equal to constant"
																};
																s === null ? s = [e] : s.push(e), c++;
															}
															var v = r === c;
														} else var v = !0;
													}
												}
											}
										} else {
											let e = {
												instancePath: t + "/reason",
												schemaPath: "#/$defs/ComparisonReason/oneOf/0/type",
												keyword: "type",
												params: { type: "object" },
												message: "must be object"
											};
											s === null ? s = [e] : s.push(e), c++;
										}
									}
									var y = l === c;
									if (y) {
										a = !0, o = 0;
										var b = !0;
									}
									let u = c;
									if (c === u) {
										if (n && typeof n == "object" && !Array.isArray(n)) {
											let e;
											if (n.kind === void 0 && (e = "kind") || n.detail === void 0 && (e = "detail")) {
												let n = {
													instancePath: t + "/reason",
													schemaPath: "#/$defs/ComparisonReason/oneOf/1/required",
													keyword: "required",
													params: { missingProperty: e },
													message: "must have required property '" + e + "'"
												};
												s === null ? s = [n] : s.push(n), c++;
											} else {
												let e = c;
												for (let e in n) if (e !== "detail" && e !== "kind") {
													let n = {
														instancePath: t + "/reason",
														schemaPath: "#/$defs/ComparisonReason/oneOf/1/additionalProperties",
														keyword: "additionalProperties",
														params: { additionalProperty: e },
														message: "must NOT have additional properties"
													};
													s === null ? s = [n] : s.push(n), c++;
													break;
												}
												if (e === c) {
													if (n.detail !== void 0) {
														let e = c;
														if (typeof n.detail != "string") {
															let e = {
																instancePath: t + "/reason/detail",
																schemaPath: "#/$defs/ComparisonReason/oneOf/1/properties/detail/type",
																keyword: "type",
																params: { type: "string" },
																message: "must be string"
															};
															s === null ? s = [e] : s.push(e), c++;
														}
														var x = e === c;
													} else var x = !0;
													if (x) {
														if (n.kind !== void 0) {
															let e = n.kind, r = c;
															if (typeof e != "string") {
																let e = {
																	instancePath: t + "/reason/kind",
																	schemaPath: "#/$defs/ComparisonReason/oneOf/1/properties/kind/type",
																	keyword: "type",
																	params: { type: "string" },
																	message: "must be string"
																};
																s === null ? s = [e] : s.push(e), c++;
															}
															if (e !== "unavailable") {
																let e = {
																	instancePath: t + "/reason/kind",
																	schemaPath: "#/$defs/ComparisonReason/oneOf/1/properties/kind/const",
																	keyword: "const",
																	params: { allowedValue: "unavailable" },
																	message: "must be equal to constant"
																};
																s === null ? s = [e] : s.push(e), c++;
															}
															var x = r === c;
														} else var x = !0;
													}
												}
											}
										} else {
											let e = {
												instancePath: t + "/reason",
												schemaPath: "#/$defs/ComparisonReason/oneOf/1/type",
												keyword: "type",
												params: { type: "object" },
												message: "must be object"
											};
											s === null ? s = [e] : s.push(e), c++;
										}
									}
									var y = u === c;
									if (y && a) a = !1, o = [o, 1];
									else {
										y && (a = !0, o = 1, b !== !0 && (b = !0));
										let e = c;
										if (c === e) {
											if (n && typeof n == "object" && !Array.isArray(n)) {
												let e;
												if (n.kind === void 0 && (e = "kind")) {
													let n = {
														instancePath: t + "/reason",
														schemaPath: "#/$defs/ComparisonReason/oneOf/2/required",
														keyword: "required",
														params: { missingProperty: e },
														message: "must have required property '" + e + "'"
													};
													s === null ? s = [n] : s.push(n), c++;
												} else {
													let e = c;
													for (let e in n) if (e !== "kind") {
														let n = {
															instancePath: t + "/reason",
															schemaPath: "#/$defs/ComparisonReason/oneOf/2/additionalProperties",
															keyword: "additionalProperties",
															params: { additionalProperty: e },
															message: "must NOT have additional properties"
														};
														s === null ? s = [n] : s.push(n), c++;
														break;
													}
													if (e === c && n.kind !== void 0) {
														let e = n.kind;
														if (typeof e != "string") {
															let e = {
																instancePath: t + "/reason/kind",
																schemaPath: "#/$defs/ComparisonReason/oneOf/2/properties/kind/type",
																keyword: "type",
																params: { type: "string" },
																message: "must be string"
															};
															s === null ? s = [e] : s.push(e), c++;
														}
														if (e !== "missing_field") {
															let e = {
																instancePath: t + "/reason/kind",
																schemaPath: "#/$defs/ComparisonReason/oneOf/2/properties/kind/const",
																keyword: "const",
																params: { allowedValue: "missing_field" },
																message: "must be equal to constant"
															};
															s === null ? s = [e] : s.push(e), c++;
														}
													}
												}
											} else {
												let e = {
													instancePath: t + "/reason",
													schemaPath: "#/$defs/ComparisonReason/oneOf/2/type",
													keyword: "type",
													params: { type: "object" },
													message: "must be object"
												};
												s === null ? s = [e] : s.push(e), c++;
											}
										}
										var y = e === c;
										if (y && a) a = !1, o = [o, 2];
										else {
											y && (a = !0, o = 2, b !== !0 && (b = !0));
											let e = c;
											if (c === e) {
												if (n && typeof n == "object" && !Array.isArray(n)) {
													let e;
													if (n.kind === void 0 && (e = "kind")) {
														let n = {
															instancePath: t + "/reason",
															schemaPath: "#/$defs/ComparisonReason/oneOf/3/required",
															keyword: "required",
															params: { missingProperty: e },
															message: "must have required property '" + e + "'"
														};
														s === null ? s = [n] : s.push(n), c++;
													} else {
														let e = c;
														for (let e in n) if (e !== "kind") {
															let n = {
																instancePath: t + "/reason",
																schemaPath: "#/$defs/ComparisonReason/oneOf/3/additionalProperties",
																keyword: "additionalProperties",
																params: { additionalProperty: e },
																message: "must NOT have additional properties"
															};
															s === null ? s = [n] : s.push(n), c++;
															break;
														}
														if (e === c && n.kind !== void 0) {
															let e = n.kind;
															if (typeof e != "string") {
																let e = {
																	instancePath: t + "/reason/kind",
																	schemaPath: "#/$defs/ComparisonReason/oneOf/3/properties/kind/type",
																	keyword: "type",
																	params: { type: "string" },
																	message: "must be string"
																};
																s === null ? s = [e] : s.push(e), c++;
															}
															if (e !== "validity") {
																let e = {
																	instancePath: t + "/reason/kind",
																	schemaPath: "#/$defs/ComparisonReason/oneOf/3/properties/kind/const",
																	keyword: "const",
																	params: { allowedValue: "validity" },
																	message: "must be equal to constant"
																};
																s === null ? s = [e] : s.push(e), c++;
															}
														}
													}
												} else {
													let e = {
														instancePath: t + "/reason",
														schemaPath: "#/$defs/ComparisonReason/oneOf/3/type",
														keyword: "type",
														params: { type: "object" },
														message: "must be object"
													};
													s === null ? s = [e] : s.push(e), c++;
												}
											}
											var y = e === c;
											if (y && a) a = !1, o = [o, 3];
											else {
												y && (a = !0, o = 3, b !== !0 && (b = !0));
												let e = c;
												if (c === e) {
													if (n && typeof n == "object" && !Array.isArray(n)) {
														let e;
														if (n.kind === void 0 && (e = "kind") || n.detail === void 0 && (e = "detail")) {
															let n = {
																instancePath: t + "/reason",
																schemaPath: "#/$defs/ComparisonReason/oneOf/4/required",
																keyword: "required",
																params: { missingProperty: e },
																message: "must have required property '" + e + "'"
															};
															s === null ? s = [n] : s.push(n), c++;
														} else {
															let e = c;
															for (let e in n) if (e !== "detail" && e !== "kind") {
																let n = {
																	instancePath: t + "/reason",
																	schemaPath: "#/$defs/ComparisonReason/oneOf/4/additionalProperties",
																	keyword: "additionalProperties",
																	params: { additionalProperty: e },
																	message: "must NOT have additional properties"
																};
																s === null ? s = [n] : s.push(n), c++;
																break;
															}
															if (e === c) {
																if (n.detail !== void 0) {
																	let e = c;
																	if (typeof n.detail != "string") {
																		let e = {
																			instancePath: t + "/reason/detail",
																			schemaPath: "#/$defs/ComparisonReason/oneOf/4/properties/detail/type",
																			keyword: "type",
																			params: { type: "string" },
																			message: "must be string"
																		};
																		s === null ? s = [e] : s.push(e), c++;
																	}
																	var S = e === c;
																} else var S = !0;
																if (S) {
																	if (n.kind !== void 0) {
																		let e = n.kind, r = c;
																		if (typeof e != "string") {
																			let e = {
																				instancePath: t + "/reason/kind",
																				schemaPath: "#/$defs/ComparisonReason/oneOf/4/properties/kind/type",
																				keyword: "type",
																				params: { type: "string" },
																				message: "must be string"
																			};
																			s === null ? s = [e] : s.push(e), c++;
																		}
																		if (e !== "resource_limit") {
																			let e = {
																				instancePath: t + "/reason/kind",
																				schemaPath: "#/$defs/ComparisonReason/oneOf/4/properties/kind/const",
																				keyword: "const",
																				params: { allowedValue: "resource_limit" },
																				message: "must be equal to constant"
																			};
																			s === null ? s = [e] : s.push(e), c++;
																		}
																		var S = r === c;
																	} else var S = !0;
																}
															}
														}
													} else {
														let e = {
															instancePath: t + "/reason",
															schemaPath: "#/$defs/ComparisonReason/oneOf/4/type",
															keyword: "type",
															params: { type: "object" },
															message: "must be object"
														};
														s === null ? s = [e] : s.push(e), c++;
													}
												}
												var y = e === c;
												y && a ? (a = !1, o = [o, 4]) : y && (a = !0, o = 4, b !== !0 && (b = !0));
											}
										}
									}
									if (a) c = i, s !== null && (i ? s.length = i : s = null);
									else {
										let e = {
											instancePath: t + "/reason",
											schemaPath: "#/$defs/ComparisonReason/oneOf",
											keyword: "oneOf",
											params: { passingSchemas: o },
											message: "must match exactly one schema in oneOf"
										};
										s === null ? s = [e] : s.push(e), c++;
									}
									var _ = r === c;
								} else var _ = !0;
							}
						}
					}
				} else {
					let e = {
						instancePath: t,
						schemaPath: "#/oneOf/2/type",
						keyword: "type",
						params: { type: "object" },
						message: "must be object"
					};
					s === null ? s = [e] : s.push(e), c++;
				}
			}
			var m = n === c;
			m && d ? (d = !1, f = [f, 2]) : m && (d = !0, f = 2, h !== !0 && (h = !0));
		}
		if (d) c = u, s !== null && (u ? s.length = u : s = null);
		else {
			let e = {
				instancePath: t,
				schemaPath: "#/oneOf",
				keyword: "oneOf",
				params: { passingSchemas: f },
				message: "must match exactly one schema in oneOf"
			};
			return s === null ? s = [e] : s.push(e), c++, o.errors = s, !1;
		}
		return o.errors = s, l.props = h, c === 0;
	}
	o.evaluated = {
		dynamicProps: !0,
		dynamicItems: !1
	};
	function s(e, { instancePath: t = "", parentData: n, parentDataProperty: r, rootData: c = e, dynamicAnchors: l = {} } = {}) {
		let u = null, d = 0, f = s.evaluated;
		if (f.dynamicProps && (f.props = void 0), f.dynamicItems && (f.items = void 0), d === 0) {
			if (e && typeof e == "object" && !Array.isArray(e)) {
				let n;
				if (e.certificate === void 0 && (n = "certificate") || e.accuracy === void 0 && (n = "accuracy")) return s.errors = [{
					instancePath: t,
					schemaPath: "#/required",
					keyword: "required",
					params: { missingProperty: n },
					message: "must have required property '" + n + "'"
				}], !1;
				{
					let n = d;
					for (let n in e) if (n !== "accuracy" && n !== "certificate") return s.errors = [{
						instancePath: t,
						schemaPath: "#/additionalProperties",
						keyword: "additionalProperties",
						params: { additionalProperty: n },
						message: "must NOT have additional properties"
					}], !1;
					if (n === d) {
						if (e.accuracy !== void 0) {
							let n = d;
							o(e.accuracy, {
								instancePath: t + "/accuracy",
								parentData: e,
								parentDataProperty: "accuracy",
								rootData: c,
								dynamicAnchors: l
							}) || (u = u === null ? o.errors : u.concat(o.errors), d = u.length);
							var p = n === d;
						} else var p = !0;
						if (p) {
							if (e.certificate !== void 0) {
								let n = e.certificate, r = d;
								if (typeof n != "string" && n !== null) return s.errors = [{
									instancePath: t + "/certificate",
									schemaPath: "#/properties/certificate/type",
									keyword: "type",
									params: { type: a.properties.certificate.type },
									message: "must be string,null"
								}], !1;
								if (d === r && typeof n == "string" && i(n) < 1) return s.errors = [{
									instancePath: t + "/certificate",
									schemaPath: "#/properties/certificate/minLength",
									keyword: "minLength",
									params: { limit: 1 },
									message: "must NOT have fewer than 1 characters"
								}], !1;
								var p = r === d;
							} else var p = !0;
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
		}
		return s.errors = u, d === 0;
	}
	s.evaluated = {
		props: !0,
		dynamicProps: !1,
		dynamicItems: !1
	};
	function c(e, { instancePath: t = "", parentData: n, parentDataProperty: r, rootData: a = e, dynamicAnchors: o = {} } = {}) {
		let l = null, u = 0, d = c.evaluated;
		if (d.dynamicProps && (d.props = void 0), d.dynamicItems && (d.items = void 0), u === 0) {
			if (e && typeof e == "object" && !Array.isArray(e)) {
				let n;
				if (e.source_coordinates === void 0 && (n = "source_coordinates") || e.fields === void 0 && (n = "fields")) return c.errors = [{
					instancePath: t,
					schemaPath: "#/required",
					keyword: "required",
					params: { missingProperty: n },
					message: "must have required property '" + n + "'"
				}], !1;
				{
					let n = u;
					for (let n in e) if (n !== "fields" && n !== "source_coordinates") return c.errors = [{
						instancePath: t,
						schemaPath: "#/additionalProperties",
						keyword: "additionalProperties",
						params: { additionalProperty: n },
						message: "must NOT have additional properties"
					}], !1;
					if (n === u) {
						if (e.fields !== void 0) {
							let n = e.fields, r = u;
							if (u === r) {
								if (n && typeof n == "object" && !Array.isArray(n)) {
									for (let e in n) {
										let n = u;
										if (u === n) {
											if (typeof e == "string") {
												if (i(e) < 1) {
													let n = {
														instancePath: t + "/fields",
														schemaPath: "#/properties/fields/propertyNames/minLength",
														keyword: "minLength",
														params: { limit: 1 },
														message: "must NOT have fewer than 1 characters",
														propertyName: e
													};
													l === null ? l = [n] : l.push(n), u++;
												}
											} else {
												let n = {
													instancePath: t + "/fields",
													schemaPath: "#/properties/fields/propertyNames/type",
													keyword: "type",
													params: { type: "string" },
													message: "must be string",
													propertyName: e
												};
												l === null ? l = [n] : l.push(n), u++;
											}
										}
										var f = n === u;
										if (!f) {
											let n = {
												instancePath: t + "/fields",
												schemaPath: "#/properties/fields/propertyNames",
												keyword: "propertyNames",
												params: { propertyName: e },
												message: "property name must be valid"
											};
											return l === null ? l = [n] : l.push(n), u++, c.errors = l, !1;
										}
									}
									if (f) for (let e in n) {
										let r = u;
										if (s(n[e], {
											instancePath: t + "/fields/" + e.replace(/~/g, "~0").replace(/\//g, "~1"),
											parentData: n,
											parentDataProperty: e,
											rootData: a,
											dynamicAnchors: o
										}) || (l = l === null ? s.errors : l.concat(s.errors), u = l.length), r !== u) break;
									}
								} else return c.errors = [{
									instancePath: t + "/fields",
									schemaPath: "#/properties/fields/type",
									keyword: "type",
									params: { type: "object" },
									message: "must be object"
								}], !1;
							}
							var p = r === u;
						} else var p = !0;
						if (p) {
							if (e.source_coordinates !== void 0) {
								let n = e.source_coordinates, r = u;
								if (u === r) {
									if (Array.isArray(n)) {
										let e = n.length;
										for (let r = 0; r < e; r++) {
											let e = n[r], a = u;
											if (u === a) {
												if (typeof e == "string") {
													if (i(e) < 1) return c.errors = [{
														instancePath: t + "/source_coordinates/" + r,
														schemaPath: "#/properties/source_coordinates/items/minLength",
														keyword: "minLength",
														params: { limit: 1 },
														message: "must NOT have fewer than 1 characters"
													}], !1;
												} else return c.errors = [{
													instancePath: t + "/source_coordinates/" + r,
													schemaPath: "#/properties/source_coordinates/items/type",
													keyword: "type",
													params: { type: "string" },
													message: "must be string"
												}], !1;
											}
											if (a !== u) break;
										}
									} else return c.errors = [{
										instancePath: t + "/source_coordinates",
										schemaPath: "#/properties/source_coordinates/type",
										keyword: "type",
										params: { type: "array" },
										message: "must be array"
									}], !1;
								}
								var p = r === u;
							} else var p = !0;
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
		return c.errors = l, u === 0;
	}
	c.evaluated = {
		props: !0,
		dynamicProps: !1,
		dynamicItems: !1
	};
	function l(e, { instancePath: t = "", parentData: n, parentDataProperty: i, rootData: a = e, dynamicAnchors: o = {} } = {}) {
		let s = null, u = 0, d = l.evaluated;
		if (d.dynamicProps && (d.props = void 0), d.dynamicItems && (d.items = void 0), u === 0) {
			if (e && typeof e == "object" && !Array.isArray(e)) {
				let n;
				if (e.schema_version === void 0 && (n = "schema_version") || e.mode === void 0 && (n = "mode") || e.profiles === void 0 && (n = "profiles") || e.front === void 0 && (n = "front") || e.counts === void 0 && (n = "counts")) return l.errors = [{
					instancePath: t,
					schemaPath: "#/required",
					keyword: "required",
					params: { missingProperty: n },
					message: "must have required property '" + n + "'"
				}], !1;
				{
					let n = u;
					for (let n in e) if (n !== "counts" && n !== "front" && n !== "mode" && n !== "pairs" && n !== "profiles" && n !== "schema_version") return l.errors = [{
						instancePath: t,
						schemaPath: "#/additionalProperties",
						keyword: "additionalProperties",
						params: { additionalProperty: n },
						message: "must NOT have additional properties"
					}], !1;
					if (n === u) {
						if (e.counts !== void 0) {
							let n = e.counts, r = u;
							if (u === u) {
								if (n && typeof n == "object" && !Array.isArray(n)) {
									let e;
									if (n.Dominated === void 0 && (e = "Dominated") || n.Equivalent === void 0 && (e = "Equivalent") || n.Incomparable === void 0 && (e = "Incomparable") || n.Unknown === void 0 && (e = "Unknown")) return l.errors = [{
										instancePath: t + "/counts",
										schemaPath: "#/$defs/ComparisonCounts/required",
										keyword: "required",
										params: { missingProperty: e },
										message: "must have required property '" + e + "'"
									}], !1;
									{
										let e = u;
										for (let e in n) if (e !== "Dominated" && e !== "Equivalent" && e !== "Incomparable" && e !== "Unknown") return l.errors = [{
											instancePath: t + "/counts",
											schemaPath: "#/$defs/ComparisonCounts/additionalProperties",
											keyword: "additionalProperties",
											params: { additionalProperty: e },
											message: "must NOT have additional properties"
										}], !1;
										if (e === u) {
											if (n.Dominated !== void 0) {
												let e = n.Dominated, r = u;
												if (typeof e != "number" || e % 1 || isNaN(e) || !isFinite(e)) return l.errors = [{
													instancePath: t + "/counts/Dominated",
													schemaPath: "#/$defs/ComparisonCounts/properties/Dominated/type",
													keyword: "type",
													params: { type: "integer" },
													message: "must be integer"
												}], !1;
												if (u === r && typeof e == "number" && isFinite(e)) {
													if (e > 9007199254740991 || isNaN(e)) return l.errors = [{
														instancePath: t + "/counts/Dominated",
														schemaPath: "#/$defs/ComparisonCounts/properties/Dominated/maximum",
														keyword: "maximum",
														params: {
															comparison: "<=",
															limit: 9007199254740991
														},
														message: "must be <= 9007199254740991"
													}], !1;
													if (e < 0 || isNaN(e)) return l.errors = [{
														instancePath: t + "/counts/Dominated",
														schemaPath: "#/$defs/ComparisonCounts/properties/Dominated/minimum",
														keyword: "minimum",
														params: {
															comparison: ">=",
															limit: 0
														},
														message: "must be >= 0"
													}], !1;
												}
												var f = r === u;
											} else var f = !0;
											if (f) {
												if (n.Equivalent !== void 0) {
													let e = n.Equivalent, r = u;
													if (typeof e != "number" || e % 1 || isNaN(e) || !isFinite(e)) return l.errors = [{
														instancePath: t + "/counts/Equivalent",
														schemaPath: "#/$defs/ComparisonCounts/properties/Equivalent/type",
														keyword: "type",
														params: { type: "integer" },
														message: "must be integer"
													}], !1;
													if (u === r && typeof e == "number" && isFinite(e)) {
														if (e > 9007199254740991 || isNaN(e)) return l.errors = [{
															instancePath: t + "/counts/Equivalent",
															schemaPath: "#/$defs/ComparisonCounts/properties/Equivalent/maximum",
															keyword: "maximum",
															params: {
																comparison: "<=",
																limit: 9007199254740991
															},
															message: "must be <= 9007199254740991"
														}], !1;
														if (e < 0 || isNaN(e)) return l.errors = [{
															instancePath: t + "/counts/Equivalent",
															schemaPath: "#/$defs/ComparisonCounts/properties/Equivalent/minimum",
															keyword: "minimum",
															params: {
																comparison: ">=",
																limit: 0
															},
															message: "must be >= 0"
														}], !1;
													}
													var f = r === u;
												} else var f = !0;
												if (f) {
													if (n.Incomparable !== void 0) {
														let e = n.Incomparable, r = u;
														if (typeof e != "number" || e % 1 || isNaN(e) || !isFinite(e)) return l.errors = [{
															instancePath: t + "/counts/Incomparable",
															schemaPath: "#/$defs/ComparisonCounts/properties/Incomparable/type",
															keyword: "type",
															params: { type: "integer" },
															message: "must be integer"
														}], !1;
														if (u === r && typeof e == "number" && isFinite(e)) {
															if (e > 9007199254740991 || isNaN(e)) return l.errors = [{
																instancePath: t + "/counts/Incomparable",
																schemaPath: "#/$defs/ComparisonCounts/properties/Incomparable/maximum",
																keyword: "maximum",
																params: {
																	comparison: "<=",
																	limit: 9007199254740991
																},
																message: "must be <= 9007199254740991"
															}], !1;
															if (e < 0 || isNaN(e)) return l.errors = [{
																instancePath: t + "/counts/Incomparable",
																schemaPath: "#/$defs/ComparisonCounts/properties/Incomparable/minimum",
																keyword: "minimum",
																params: {
																	comparison: ">=",
																	limit: 0
																},
																message: "must be >= 0"
															}], !1;
														}
														var f = r === u;
													} else var f = !0;
													if (f) {
														if (n.Unknown !== void 0) {
															let e = n.Unknown, r = u;
															if (typeof e != "number" || e % 1 || isNaN(e) || !isFinite(e)) return l.errors = [{
																instancePath: t + "/counts/Unknown",
																schemaPath: "#/$defs/ComparisonCounts/properties/Unknown/type",
																keyword: "type",
																params: { type: "integer" },
																message: "must be integer"
															}], !1;
															if (u === r && typeof e == "number" && isFinite(e)) {
																if (e > 9007199254740991 || isNaN(e)) return l.errors = [{
																	instancePath: t + "/counts/Unknown",
																	schemaPath: "#/$defs/ComparisonCounts/properties/Unknown/maximum",
																	keyword: "maximum",
																	params: {
																		comparison: "<=",
																		limit: 9007199254740991
																	},
																	message: "must be <= 9007199254740991"
																}], !1;
																if (e < 0 || isNaN(e)) return l.errors = [{
																	instancePath: t + "/counts/Unknown",
																	schemaPath: "#/$defs/ComparisonCounts/properties/Unknown/minimum",
																	keyword: "minimum",
																	params: {
																		comparison: ">=",
																		limit: 0
																	},
																	message: "must be >= 0"
																}], !1;
															}
															var f = r === u;
														} else var f = !0;
													}
												}
											}
										}
									}
								} else return l.errors = [{
									instancePath: t + "/counts",
									schemaPath: "#/$defs/ComparisonCounts/type",
									keyword: "type",
									params: { type: "object" },
									message: "must be object"
								}], !1;
							}
							var p = r === u;
						} else var p = !0;
						if (p) {
							if (e.front !== void 0) {
								let n = e.front, r = u;
								if (u === r) {
									if (Array.isArray(n)) {
										let e = n.length;
										for (let r = 0; r < e; r++) {
											let e = n[r], i = u;
											if (typeof e != "number" || e % 1 || isNaN(e) || !isFinite(e)) return l.errors = [{
												instancePath: t + "/front/" + r,
												schemaPath: "#/properties/front/items/type",
												keyword: "type",
												params: { type: "integer" },
												message: "must be integer"
											}], !1;
											if (u === i && typeof e == "number" && isFinite(e)) {
												if (e > 9007199254740991 || isNaN(e)) return l.errors = [{
													instancePath: t + "/front/" + r,
													schemaPath: "#/properties/front/items/maximum",
													keyword: "maximum",
													params: {
														comparison: "<=",
														limit: 9007199254740991
													},
													message: "must be <= 9007199254740991"
												}], !1;
												if (e < 1 || isNaN(e)) return l.errors = [{
													instancePath: t + "/front/" + r,
													schemaPath: "#/properties/front/items/minimum",
													keyword: "minimum",
													params: {
														comparison: ">=",
														limit: 1
													},
													message: "must be >= 1"
												}], !1;
											}
											if (i !== u) break;
										}
									} else return l.errors = [{
										instancePath: t + "/front",
										schemaPath: "#/properties/front/type",
										keyword: "type",
										params: { type: "array" },
										message: "must be array"
									}], !1;
								}
								var p = r === u;
							} else var p = !0;
							if (p) {
								if (e.mode !== void 0) {
									let n = e.mode, r = u;
									if (typeof n != "string") return l.errors = [{
										instancePath: t + "/mode",
										schemaPath: "#/properties/mode/type",
										keyword: "type",
										params: { type: "string" },
										message: "must be string"
									}], !1;
									if (n !== "generic") return l.errors = [{
										instancePath: t + "/mode",
										schemaPath: "#/properties/mode/const",
										keyword: "const",
										params: { allowedValue: "generic" },
										message: "must be equal to constant"
									}], !1;
									var p = r === u;
								} else var p = !0;
								if (p) {
									if (e.pairs !== void 0) {
										let n = e.pairs, i = u;
										if (u === i) {
											if (Array.isArray(n)) {
												let e = n.length;
												for (let i = 0; i < e; i++) {
													let e = u;
													if (r(n[i], {
														instancePath: t + "/pairs/" + i,
														parentData: n,
														parentDataProperty: i,
														rootData: a,
														dynamicAnchors: o
													}) || (s = s === null ? r.errors : s.concat(r.errors), u = s.length), e !== u) break;
												}
											} else return l.errors = [{
												instancePath: t + "/pairs",
												schemaPath: "#/properties/pairs/type",
												keyword: "type",
												params: { type: "array" },
												message: "must be array"
											}], !1;
										}
										var p = i === u;
									} else var p = !0;
									if (p) {
										if (e.profiles !== void 0) {
											let n = e.profiles, r = u;
											if (u === r) {
												if (Array.isArray(n)) {
													let e = n.length;
													for (let r = 0; r < e; r++) {
														let e = u;
														if (c(n[r], {
															instancePath: t + "/profiles/" + r,
															parentData: n,
															parentDataProperty: r,
															rootData: a,
															dynamicAnchors: o
														}) || (s = s === null ? c.errors : s.concat(c.errors), u = s.length), e !== u) break;
													}
												} else return l.errors = [{
													instancePath: t + "/profiles",
													schemaPath: "#/properties/profiles/type",
													keyword: "type",
													params: { type: "array" },
													message: "must be array"
												}], !1;
											}
											var p = r === u;
										} else var p = !0;
										if (p) {
											if (e.schema_version !== void 0) {
												let n = e.schema_version, r = u;
												if (typeof n != "number" || n % 1 || isNaN(n) || !isFinite(n)) return l.errors = [{
													instancePath: t + "/schema_version",
													schemaPath: "#/properties/schema_version/type",
													keyword: "type",
													params: { type: "integer" },
													message: "must be integer"
												}], !1;
												if (n !== 5) return l.errors = [{
													instancePath: t + "/schema_version",
													schemaPath: "#/properties/schema_version/const",
													keyword: "const",
													params: { allowedValue: 5 },
													message: "must be equal to constant"
												}], !1;
												if (u === r && typeof n == "number" && isFinite(n)) {
													if (n > 255 || isNaN(n)) return l.errors = [{
														instancePath: t + "/schema_version",
														schemaPath: "#/properties/schema_version/maximum",
														keyword: "maximum",
														params: {
															comparison: "<=",
															limit: 255
														},
														message: "must be <= 255"
													}], !1;
													if (n < 0 || isNaN(n)) return l.errors = [{
														instancePath: t + "/schema_version",
														schemaPath: "#/properties/schema_version/minimum",
														keyword: "minimum",
														params: {
															comparison: ">=",
															limit: 0
														},
														message: "must be >= 0"
													}], !1;
												}
												var p = r === u;
											} else var p = !0;
										}
									}
								}
							}
						}
					}
				}
			} else return l.errors = [{
				instancePath: t,
				schemaPath: "#/type",
				keyword: "type",
				params: { type: "object" },
				message: "must be object"
			}], !1;
		}
		return l.errors = s, u === 0;
	}
	l.evaluated = {
		props: !0,
		dynamicProps: !1,
		dynamicItems: !1
	};
})))().validateWasmComparison;
export { n as validateWasmComparison };
