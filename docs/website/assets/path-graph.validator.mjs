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
	e.validatePathGraph = a;
	var n = {
		$schema: "https://json-schema.org/draft/2020-12/schema",
		$id: "https://codingthrust.github.io/problem-reductions/contracts/path-graph",
		title: "PathGraphData",
		type: "object",
		additionalProperties: !1,
		required: [
			"schema_version",
			"nodes",
			"edges"
		],
		properties: {
			schema_version: {
				type: "integer",
				const: 1
			},
			nodes: {
				type: "array",
				minItems: 1,
				items: {
					type: "object",
					additionalProperties: !1,
					required: [
						"name",
						"variant",
						"default",
						"parameter_names"
					],
					properties: {
						name: {
							type: "string",
							minLength: 1
						},
						variant: {
							type: "object",
							propertyNames: { minLength: 1 },
							additionalProperties: {
								type: "string",
								minLength: 1
							}
						},
						default: { type: "boolean" },
						parameter_names: {
							type: "array",
							uniqueItems: !0,
							items: {
								type: "string",
								minLength: 1
							}
						}
					}
				}
			},
			edges: {
				type: "array",
				items: {
					type: "object",
					additionalProperties: !1,
					required: [
						"source",
						"target",
						"witness",
						"turing",
						"parameters",
						"parameter_contract_error"
					],
					properties: {
						source: {
							type: "integer",
							minimum: 0,
							maximum: 9007199254740991
						},
						target: {
							type: "integer",
							minimum: 0,
							maximum: 9007199254740991
						},
						witness: { type: "boolean" },
						turing: { type: "boolean" },
						parameter_contract_error: {
							type: ["string", "null"],
							minLength: 1
						},
						parameters: {
							type: "array",
							items: { $ref: "#/$defs/Parameter" }
						}
					}
				}
			}
		},
		$defs: { Parameter: { oneOf: [{
			type: "object",
			additionalProperties: !1,
			required: [
				"field",
				"contract",
				"formula",
				"reason"
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
				reason: { type: "null" }
			}
		}, {
			type: "object",
			additionalProperties: !1,
			required: [
				"field",
				"contract",
				"formula",
				"reason"
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
				}
			}
		}] } }
	}, r = { oneOf: [{
		type: "object",
		additionalProperties: !1,
		required: [
			"field",
			"contract",
			"formula",
			"reason"
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
			reason: { type: "null" }
		}
	}, {
		type: "object",
		additionalProperties: !1,
		required: [
			"field",
			"contract",
			"formula",
			"reason"
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
			}
		}
	}] }, i = t().default;
	function a(e, { instancePath: t = "", parentData: o, parentDataProperty: s, rootData: c = e, dynamicAnchors: l = {} } = {}) {
		let u = null, d = 0, f = a.evaluated;
		if (f.dynamicProps && (f.props = void 0), f.dynamicItems && (f.items = void 0), d === 0) {
			if (e && typeof e == "object" && !Array.isArray(e)) {
				let o;
				if (e.schema_version === void 0 && (o = "schema_version") || e.nodes === void 0 && (o = "nodes") || e.edges === void 0 && (o = "edges")) return a.errors = [{
					instancePath: t,
					schemaPath: "#/required",
					keyword: "required",
					params: { missingProperty: o },
					message: "must have required property '" + o + "'"
				}], !1;
				{
					let o = d;
					for (let n in e) if (n !== "schema_version" && n !== "nodes" && n !== "edges") return a.errors = [{
						instancePath: t,
						schemaPath: "#/additionalProperties",
						keyword: "additionalProperties",
						params: { additionalProperty: n },
						message: "must NOT have additional properties"
					}], !1;
					if (o === d) {
						if (e.schema_version !== void 0) {
							let n = e.schema_version, r = d;
							if (typeof n != "number" || n % 1 || isNaN(n) || !isFinite(n)) return a.errors = [{
								instancePath: t + "/schema_version",
								schemaPath: "#/properties/schema_version/type",
								keyword: "type",
								params: { type: "integer" },
								message: "must be integer"
							}], !1;
							if (n !== 1) return a.errors = [{
								instancePath: t + "/schema_version",
								schemaPath: "#/properties/schema_version/const",
								keyword: "const",
								params: { allowedValue: 1 },
								message: "must be equal to constant"
							}], !1;
							var p = r === d;
						} else var p = !0;
						if (p) {
							if (e.nodes !== void 0) {
								let n = e.nodes, r = d;
								if (d === r) {
									if (Array.isArray(n)) {
										if (n.length < 1) return a.errors = [{
											instancePath: t + "/nodes",
											schemaPath: "#/properties/nodes/minItems",
											keyword: "minItems",
											params: { limit: 1 },
											message: "must NOT have fewer than 1 items"
										}], !1;
										{
											let e = n.length;
											for (let r = 0; r < e; r++) {
												let e = n[r], o = d;
												if (d === o) {
													if (e && typeof e == "object" && !Array.isArray(e)) {
														let n;
														if (e.name === void 0 && (n = "name") || e.variant === void 0 && (n = "variant") || e.default === void 0 && (n = "default") || e.parameter_names === void 0 && (n = "parameter_names")) return a.errors = [{
															instancePath: t + "/nodes/" + r,
															schemaPath: "#/properties/nodes/items/required",
															keyword: "required",
															params: { missingProperty: n },
															message: "must have required property '" + n + "'"
														}], !1;
														{
															let n = d;
															for (let n in e) if (n !== "name" && n !== "variant" && n !== "default" && n !== "parameter_names") return a.errors = [{
																instancePath: t + "/nodes/" + r,
																schemaPath: "#/properties/nodes/items/additionalProperties",
																keyword: "additionalProperties",
																params: { additionalProperty: n },
																message: "must NOT have additional properties"
															}], !1;
															if (n === d) {
																if (e.name !== void 0) {
																	let n = e.name, o = d;
																	if (d === o) {
																		if (typeof n == "string") {
																			if (i(n) < 1) return a.errors = [{
																				instancePath: t + "/nodes/" + r + "/name",
																				schemaPath: "#/properties/nodes/items/properties/name/minLength",
																				keyword: "minLength",
																				params: { limit: 1 },
																				message: "must NOT have fewer than 1 characters"
																			}], !1;
																		} else return a.errors = [{
																			instancePath: t + "/nodes/" + r + "/name",
																			schemaPath: "#/properties/nodes/items/properties/name/type",
																			keyword: "type",
																			params: { type: "string" },
																			message: "must be string"
																		}], !1;
																	}
																	var m = o === d;
																} else var m = !0;
																if (m) {
																	if (e.variant !== void 0) {
																		let n = e.variant, o = d;
																		if (d === o) {
																			if (n && typeof n == "object" && !Array.isArray(n)) {
																				for (let e in n) {
																					let n = d;
																					if (typeof e == "string" && i(e) < 1) {
																						let n = {
																							instancePath: t + "/nodes/" + r + "/variant",
																							schemaPath: "#/properties/nodes/items/properties/variant/propertyNames/minLength",
																							keyword: "minLength",
																							params: { limit: 1 },
																							message: "must NOT have fewer than 1 characters",
																							propertyName: e
																						};
																						u === null ? u = [n] : u.push(n), d++;
																					}
																					var h = n === d;
																					if (!h) {
																						let n = {
																							instancePath: t + "/nodes/" + r + "/variant",
																							schemaPath: "#/properties/nodes/items/properties/variant/propertyNames",
																							keyword: "propertyNames",
																							params: { propertyName: e },
																							message: "property name must be valid"
																						};
																						return u === null ? u = [n] : u.push(n), d++, a.errors = u, !1;
																					}
																				}
																				if (h) for (let e in n) {
																					let o = n[e], s = d;
																					if (d === s) {
																						if (typeof o == "string") {
																							if (i(o) < 1) return a.errors = [{
																								instancePath: t + "/nodes/" + r + "/variant/" + e.replace(/~/g, "~0").replace(/\//g, "~1"),
																								schemaPath: "#/properties/nodes/items/properties/variant/additionalProperties/minLength",
																								keyword: "minLength",
																								params: { limit: 1 },
																								message: "must NOT have fewer than 1 characters"
																							}], !1;
																						} else return a.errors = [{
																							instancePath: t + "/nodes/" + r + "/variant/" + e.replace(/~/g, "~0").replace(/\//g, "~1"),
																							schemaPath: "#/properties/nodes/items/properties/variant/additionalProperties/type",
																							keyword: "type",
																							params: { type: "string" },
																							message: "must be string"
																						}], !1;
																					}
																					if (s !== d) break;
																				}
																			} else return a.errors = [{
																				instancePath: t + "/nodes/" + r + "/variant",
																				schemaPath: "#/properties/nodes/items/properties/variant/type",
																				keyword: "type",
																				params: { type: "object" },
																				message: "must be object"
																			}], !1;
																		}
																		var m = o === d;
																	} else var m = !0;
																	if (m) {
																		if (e.default !== void 0) {
																			let n = d;
																			if (typeof e.default != "boolean") return a.errors = [{
																				instancePath: t + "/nodes/" + r + "/default",
																				schemaPath: "#/properties/nodes/items/properties/default/type",
																				keyword: "type",
																				params: { type: "boolean" },
																				message: "must be boolean"
																			}], !1;
																			var m = n === d;
																		} else var m = !0;
																		if (m) {
																			if (e.parameter_names !== void 0) {
																				let n = e.parameter_names, o = d;
																				if (d === o) {
																					if (Array.isArray(n)) {
																						var g = !0;
																						let e = n.length;
																						for (let o = 0; o < e; o++) {
																							let e = n[o], s = d;
																							if (d === s) {
																								if (typeof e == "string") {
																									if (i(e) < 1) return a.errors = [{
																										instancePath: t + "/nodes/" + r + "/parameter_names/" + o,
																										schemaPath: "#/properties/nodes/items/properties/parameter_names/items/minLength",
																										keyword: "minLength",
																										params: { limit: 1 },
																										message: "must NOT have fewer than 1 characters"
																									}], !1;
																								} else return a.errors = [{
																									instancePath: t + "/nodes/" + r + "/parameter_names/" + o,
																									schemaPath: "#/properties/nodes/items/properties/parameter_names/items/type",
																									keyword: "type",
																									params: { type: "string" },
																									message: "must be string"
																								}], !1;
																							}
																							var g = s === d;
																							if (!g) break;
																						}
																						if (g) {
																							let e = n.length, i;
																							if (e > 1) {
																								let o = {};
																								for (; e--;) {
																									let s = n[e];
																									if (typeof s == "string") {
																										if (typeof o[s] == "number") return i = o[s], a.errors = [{
																											instancePath: t + "/nodes/" + r + "/parameter_names",
																											schemaPath: "#/properties/nodes/items/properties/parameter_names/uniqueItems",
																											keyword: "uniqueItems",
																											params: {
																												i: e,
																												j: i
																											},
																											message: "must NOT have duplicate items (items ## " + i + " and " + e + " are identical)"
																										}], !1;
																										o[s] = e;
																									}
																								}
																							}
																						}
																					} else return a.errors = [{
																						instancePath: t + "/nodes/" + r + "/parameter_names",
																						schemaPath: "#/properties/nodes/items/properties/parameter_names/type",
																						keyword: "type",
																						params: { type: "array" },
																						message: "must be array"
																					}], !1;
																				}
																				var m = o === d;
																			} else var m = !0;
																		}
																	}
																}
															}
														}
													} else return a.errors = [{
														instancePath: t + "/nodes/" + r,
														schemaPath: "#/properties/nodes/items/type",
														keyword: "type",
														params: { type: "object" },
														message: "must be object"
													}], !1;
												}
												if (o !== d) break;
											}
										}
									} else return a.errors = [{
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
									let o = e.edges, s = d;
									if (d === s) {
										if (Array.isArray(o)) {
											let e = o.length;
											for (let s = 0; s < e; s++) {
												let e = o[s], c = d;
												if (d === c) {
													if (e && typeof e == "object" && !Array.isArray(e)) {
														let o;
														if (e.source === void 0 && (o = "source") || e.target === void 0 && (o = "target") || e.witness === void 0 && (o = "witness") || e.turing === void 0 && (o = "turing") || e.parameters === void 0 && (o = "parameters") || e.parameter_contract_error === void 0 && (o = "parameter_contract_error")) return a.errors = [{
															instancePath: t + "/edges/" + s,
															schemaPath: "#/properties/edges/items/required",
															keyword: "required",
															params: { missingProperty: o },
															message: "must have required property '" + o + "'"
														}], !1;
														{
															let o = d;
															for (let n in e) if (n !== "source" && n !== "target" && n !== "witness" && n !== "turing" && n !== "parameter_contract_error" && n !== "parameters") return a.errors = [{
																instancePath: t + "/edges/" + s,
																schemaPath: "#/properties/edges/items/additionalProperties",
																keyword: "additionalProperties",
																params: { additionalProperty: n },
																message: "must NOT have additional properties"
															}], !1;
															if (o === d) {
																if (e.source !== void 0) {
																	let n = e.source, r = d;
																	if (typeof n != "number" || n % 1 || isNaN(n) || !isFinite(n)) return a.errors = [{
																		instancePath: t + "/edges/" + s + "/source",
																		schemaPath: "#/properties/edges/items/properties/source/type",
																		keyword: "type",
																		params: { type: "integer" },
																		message: "must be integer"
																	}], !1;
																	if (d === r && typeof n == "number" && isFinite(n)) {
																		if (n > 9007199254740991 || isNaN(n)) return a.errors = [{
																			instancePath: t + "/edges/" + s + "/source",
																			schemaPath: "#/properties/edges/items/properties/source/maximum",
																			keyword: "maximum",
																			params: {
																				comparison: "<=",
																				limit: 9007199254740991
																			},
																			message: "must be <= 9007199254740991"
																		}], !1;
																		if (n < 0 || isNaN(n)) return a.errors = [{
																			instancePath: t + "/edges/" + s + "/source",
																			schemaPath: "#/properties/edges/items/properties/source/minimum",
																			keyword: "minimum",
																			params: {
																				comparison: ">=",
																				limit: 0
																			},
																			message: "must be >= 0"
																		}], !1;
																	}
																	var _ = r === d;
																} else var _ = !0;
																if (_) {
																	if (e.target !== void 0) {
																		let n = e.target, r = d;
																		if (typeof n != "number" || n % 1 || isNaN(n) || !isFinite(n)) return a.errors = [{
																			instancePath: t + "/edges/" + s + "/target",
																			schemaPath: "#/properties/edges/items/properties/target/type",
																			keyword: "type",
																			params: { type: "integer" },
																			message: "must be integer"
																		}], !1;
																		if (d === r && typeof n == "number" && isFinite(n)) {
																			if (n > 9007199254740991 || isNaN(n)) return a.errors = [{
																				instancePath: t + "/edges/" + s + "/target",
																				schemaPath: "#/properties/edges/items/properties/target/maximum",
																				keyword: "maximum",
																				params: {
																					comparison: "<=",
																					limit: 9007199254740991
																				},
																				message: "must be <= 9007199254740991"
																			}], !1;
																			if (n < 0 || isNaN(n)) return a.errors = [{
																				instancePath: t + "/edges/" + s + "/target",
																				schemaPath: "#/properties/edges/items/properties/target/minimum",
																				keyword: "minimum",
																				params: {
																					comparison: ">=",
																					limit: 0
																				},
																				message: "must be >= 0"
																			}], !1;
																		}
																		var _ = r === d;
																	} else var _ = !0;
																	if (_) {
																		if (e.witness !== void 0) {
																			let n = d;
																			if (typeof e.witness != "boolean") return a.errors = [{
																				instancePath: t + "/edges/" + s + "/witness",
																				schemaPath: "#/properties/edges/items/properties/witness/type",
																				keyword: "type",
																				params: { type: "boolean" },
																				message: "must be boolean"
																			}], !1;
																			var _ = n === d;
																		} else var _ = !0;
																		if (_) {
																			if (e.turing !== void 0) {
																				let n = d;
																				if (typeof e.turing != "boolean") return a.errors = [{
																					instancePath: t + "/edges/" + s + "/turing",
																					schemaPath: "#/properties/edges/items/properties/turing/type",
																					keyword: "type",
																					params: { type: "boolean" },
																					message: "must be boolean"
																				}], !1;
																				var _ = n === d;
																			} else var _ = !0;
																			if (_) {
																				if (e.parameter_contract_error !== void 0) {
																					let r = e.parameter_contract_error, o = d;
																					if (typeof r != "string" && r !== null) return a.errors = [{
																						instancePath: t + "/edges/" + s + "/parameter_contract_error",
																						schemaPath: "#/properties/edges/items/properties/parameter_contract_error/type",
																						keyword: "type",
																						params: { type: n.properties.edges.items.properties.parameter_contract_error.type },
																						message: "must be string,null"
																					}], !1;
																					if (d === o && typeof r == "string" && i(r) < 1) return a.errors = [{
																						instancePath: t + "/edges/" + s + "/parameter_contract_error",
																						schemaPath: "#/properties/edges/items/properties/parameter_contract_error/minLength",
																						keyword: "minLength",
																						params: { limit: 1 },
																						message: "must NOT have fewer than 1 characters"
																					}], !1;
																					var _ = o === d;
																				} else var _ = !0;
																				if (_) {
																					if (e.parameters !== void 0) {
																						let n = e.parameters, o = d;
																						if (d === o) {
																							if (Array.isArray(n)) {
																								let e = n.length;
																								for (let o = 0; o < e; o++) {
																									let e = n[o], c = d, l = d, f = !1, p = null, m = d;
																									if (d === m) {
																										if (e && typeof e == "object" && !Array.isArray(e)) {
																											let n;
																											if (e.field === void 0 && (n = "field") || e.contract === void 0 && (n = "contract") || e.formula === void 0 && (n = "formula") || e.reason === void 0 && (n = "reason")) {
																												let e = {
																													instancePath: t + "/edges/" + s + "/parameters/" + o,
																													schemaPath: "#/$defs/Parameter/oneOf/0/required",
																													keyword: "required",
																													params: { missingProperty: n },
																													message: "must have required property '" + n + "'"
																												};
																												u === null ? u = [e] : u.push(e), d++;
																											} else {
																												let n = d;
																												for (let n in e) if (n !== "field" && n !== "contract" && n !== "formula" && n !== "reason") {
																													let e = {
																														instancePath: t + "/edges/" + s + "/parameters/" + o,
																														schemaPath: "#/$defs/Parameter/oneOf/0/additionalProperties",
																														keyword: "additionalProperties",
																														params: { additionalProperty: n },
																														message: "must NOT have additional properties"
																													};
																													u === null ? u = [e] : u.push(e), d++;
																													break;
																												}
																												if (n === d) {
																													if (e.field !== void 0) {
																														let n = e.field, r = d;
																														if (d === r) {
																															if (typeof n == "string") {
																																if (i(n) < 1) {
																																	let e = {
																																		instancePath: t + "/edges/" + s + "/parameters/" + o + "/field",
																																		schemaPath: "#/$defs/Parameter/oneOf/0/properties/field/minLength",
																																		keyword: "minLength",
																																		params: { limit: 1 },
																																		message: "must NOT have fewer than 1 characters"
																																	};
																																	u === null ? u = [e] : u.push(e), d++;
																																}
																															} else {
																																let e = {
																																	instancePath: t + "/edges/" + s + "/parameters/" + o + "/field",
																																	schemaPath: "#/$defs/Parameter/oneOf/0/properties/field/type",
																																	keyword: "type",
																																	params: { type: "string" },
																																	message: "must be string"
																																};
																																u === null ? u = [e] : u.push(e), d++;
																															}
																														}
																														var v = r === d;
																													} else var v = !0;
																													if (v) {
																														if (e.contract !== void 0) {
																															let n = e.contract, i = d;
																															if (typeof n != "string") {
																																let e = {
																																	instancePath: t + "/edges/" + s + "/parameters/" + o + "/contract",
																																	schemaPath: "#/$defs/Parameter/oneOf/0/properties/contract/type",
																																	keyword: "type",
																																	params: { type: "string" },
																																	message: "must be string"
																																};
																																u === null ? u = [e] : u.push(e), d++;
																															}
																															if (n !== "exact" && n !== "upper_bound") {
																																let e = {
																																	instancePath: t + "/edges/" + s + "/parameters/" + o + "/contract",
																																	schemaPath: "#/$defs/Parameter/oneOf/0/properties/contract/enum",
																																	keyword: "enum",
																																	params: { allowedValues: r.oneOf[0].properties.contract.enum },
																																	message: "must be equal to one of the allowed values"
																																};
																																u === null ? u = [e] : u.push(e), d++;
																															}
																															var v = i === d;
																														} else var v = !0;
																														if (v) {
																															if (e.formula !== void 0) {
																																let n = e.formula, r = d;
																																if (d === r) {
																																	if (typeof n == "string") {
																																		if (i(n) < 1) {
																																			let e = {
																																				instancePath: t + "/edges/" + s + "/parameters/" + o + "/formula",
																																				schemaPath: "#/$defs/Parameter/oneOf/0/properties/formula/minLength",
																																				keyword: "minLength",
																																				params: { limit: 1 },
																																				message: "must NOT have fewer than 1 characters"
																																			};
																																			u === null ? u = [e] : u.push(e), d++;
																																		}
																																	} else {
																																		let e = {
																																			instancePath: t + "/edges/" + s + "/parameters/" + o + "/formula",
																																			schemaPath: "#/$defs/Parameter/oneOf/0/properties/formula/type",
																																			keyword: "type",
																																			params: { type: "string" },
																																			message: "must be string"
																																		};
																																		u === null ? u = [e] : u.push(e), d++;
																																	}
																																}
																																var v = r === d;
																															} else var v = !0;
																															if (v) {
																																if (e.reason !== void 0) {
																																	let n = d;
																																	if (e.reason !== null) {
																																		let e = {
																																			instancePath: t + "/edges/" + s + "/parameters/" + o + "/reason",
																																			schemaPath: "#/$defs/Parameter/oneOf/0/properties/reason/type",
																																			keyword: "type",
																																			params: { type: "null" },
																																			message: "must be null"
																																		};
																																		u === null ? u = [e] : u.push(e), d++;
																																	}
																																	var v = n === d;
																																} else var v = !0;
																															}
																														}
																													}
																												}
																											}
																										} else {
																											let e = {
																												instancePath: t + "/edges/" + s + "/parameters/" + o,
																												schemaPath: "#/$defs/Parameter/oneOf/0/type",
																												keyword: "type",
																												params: { type: "object" },
																												message: "must be object"
																											};
																											u === null ? u = [e] : u.push(e), d++;
																										}
																									}
																									var y = m === d;
																									if (y) {
																										f = !0, p = 0;
																										var b = !0;
																									}
																									let h = d;
																									if (d === h) {
																										if (e && typeof e == "object" && !Array.isArray(e)) {
																											let n;
																											if (e.field === void 0 && (n = "field") || e.contract === void 0 && (n = "contract") || e.formula === void 0 && (n = "formula") || e.reason === void 0 && (n = "reason")) {
																												let e = {
																													instancePath: t + "/edges/" + s + "/parameters/" + o,
																													schemaPath: "#/$defs/Parameter/oneOf/1/required",
																													keyword: "required",
																													params: { missingProperty: n },
																													message: "must have required property '" + n + "'"
																												};
																												u === null ? u = [e] : u.push(e), d++;
																											} else {
																												let n = d;
																												for (let n in e) if (n !== "field" && n !== "contract" && n !== "formula" && n !== "reason") {
																													let e = {
																														instancePath: t + "/edges/" + s + "/parameters/" + o,
																														schemaPath: "#/$defs/Parameter/oneOf/1/additionalProperties",
																														keyword: "additionalProperties",
																														params: { additionalProperty: n },
																														message: "must NOT have additional properties"
																													};
																													u === null ? u = [e] : u.push(e), d++;
																													break;
																												}
																												if (n === d) {
																													if (e.field !== void 0) {
																														let n = e.field, r = d;
																														if (d === r) {
																															if (typeof n == "string") {
																																if (i(n) < 1) {
																																	let e = {
																																		instancePath: t + "/edges/" + s + "/parameters/" + o + "/field",
																																		schemaPath: "#/$defs/Parameter/oneOf/1/properties/field/minLength",
																																		keyword: "minLength",
																																		params: { limit: 1 },
																																		message: "must NOT have fewer than 1 characters"
																																	};
																																	u === null ? u = [e] : u.push(e), d++;
																																}
																															} else {
																																let e = {
																																	instancePath: t + "/edges/" + s + "/parameters/" + o + "/field",
																																	schemaPath: "#/$defs/Parameter/oneOf/1/properties/field/type",
																																	keyword: "type",
																																	params: { type: "string" },
																																	message: "must be string"
																																};
																																u === null ? u = [e] : u.push(e), d++;
																															}
																														}
																														var x = r === d;
																													} else var x = !0;
																													if (x) {
																														if (e.contract !== void 0) {
																															let n = e.contract, r = d;
																															if (typeof n != "string") {
																																let e = {
																																	instancePath: t + "/edges/" + s + "/parameters/" + o + "/contract",
																																	schemaPath: "#/$defs/Parameter/oneOf/1/properties/contract/type",
																																	keyword: "type",
																																	params: { type: "string" },
																																	message: "must be string"
																																};
																																u === null ? u = [e] : u.push(e), d++;
																															}
																															if (n !== "unavailable") {
																																let e = {
																																	instancePath: t + "/edges/" + s + "/parameters/" + o + "/contract",
																																	schemaPath: "#/$defs/Parameter/oneOf/1/properties/contract/const",
																																	keyword: "const",
																																	params: { allowedValue: "unavailable" },
																																	message: "must be equal to constant"
																																};
																																u === null ? u = [e] : u.push(e), d++;
																															}
																															var x = r === d;
																														} else var x = !0;
																														if (x) {
																															if (e.formula !== void 0) {
																																let n = d;
																																if (e.formula !== null) {
																																	let e = {
																																		instancePath: t + "/edges/" + s + "/parameters/" + o + "/formula",
																																		schemaPath: "#/$defs/Parameter/oneOf/1/properties/formula/type",
																																		keyword: "type",
																																		params: { type: "null" },
																																		message: "must be null"
																																	};
																																	u === null ? u = [e] : u.push(e), d++;
																																}
																																var x = n === d;
																															} else var x = !0;
																															if (x) {
																																if (e.reason !== void 0) {
																																	let n = e.reason, r = d;
																																	if (d === r) {
																																		if (typeof n == "string") {
																																			if (i(n) < 1) {
																																				let e = {
																																					instancePath: t + "/edges/" + s + "/parameters/" + o + "/reason",
																																					schemaPath: "#/$defs/Parameter/oneOf/1/properties/reason/minLength",
																																					keyword: "minLength",
																																					params: { limit: 1 },
																																					message: "must NOT have fewer than 1 characters"
																																				};
																																				u === null ? u = [e] : u.push(e), d++;
																																			}
																																		} else {
																																			let e = {
																																				instancePath: t + "/edges/" + s + "/parameters/" + o + "/reason",
																																				schemaPath: "#/$defs/Parameter/oneOf/1/properties/reason/type",
																																				keyword: "type",
																																				params: { type: "string" },
																																				message: "must be string"
																																			};
																																			u === null ? u = [e] : u.push(e), d++;
																																		}
																																	}
																																	var x = r === d;
																																} else var x = !0;
																															}
																														}
																													}
																												}
																											}
																										} else {
																											let e = {
																												instancePath: t + "/edges/" + s + "/parameters/" + o,
																												schemaPath: "#/$defs/Parameter/oneOf/1/type",
																												keyword: "type",
																												params: { type: "object" },
																												message: "must be object"
																											};
																											u === null ? u = [e] : u.push(e), d++;
																										}
																									}
																									var y = h === d;
																									if (y && f ? (f = !1, p = [p, 1]) : y && (f = !0, p = 1, b !== !0 && (b = !0)), f) d = l, u !== null && (l ? u.length = l : u = null);
																									else {
																										let e = {
																											instancePath: t + "/edges/" + s + "/parameters/" + o,
																											schemaPath: "#/$defs/Parameter/oneOf",
																											keyword: "oneOf",
																											params: { passingSchemas: p },
																											message: "must match exactly one schema in oneOf"
																										};
																										return u === null ? u = [e] : u.push(e), d++, a.errors = u, !1;
																									}
																									if (c !== d) break;
																								}
																							} else return a.errors = [{
																								instancePath: t + "/edges/" + s + "/parameters",
																								schemaPath: "#/properties/edges/items/properties/parameters/type",
																								keyword: "type",
																								params: { type: "array" },
																								message: "must be array"
																							}], !1;
																						}
																						var _ = o === d;
																					} else var _ = !0;
																				}
																			}
																		}
																	}
																}
															}
														}
													} else return a.errors = [{
														instancePath: t + "/edges/" + s,
														schemaPath: "#/properties/edges/items/type",
														keyword: "type",
														params: { type: "object" },
														message: "must be object"
													}], !1;
												}
												if (c !== d) break;
											}
										} else return a.errors = [{
											instancePath: t + "/edges",
											schemaPath: "#/properties/edges/type",
											keyword: "type",
											params: { type: "array" },
											message: "must be array"
										}], !1;
									}
									var p = s === d;
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
})))().validatePathGraph;
export { n as validatePathGraph };
