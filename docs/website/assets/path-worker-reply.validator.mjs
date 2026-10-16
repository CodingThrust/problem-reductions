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
	e.validatePathReplyEnvelope = i;
	var n = {
		$schema: "https://json-schema.org/draft/2020-12/schema",
		$id: "https://codingthrust.github.io/problem-reductions/contracts/path-worker-reply",
		title: "PathReplyEnvelope",
		oneOf: [
			{
				type: "object",
				additionalProperties: !1,
				required: [
					"schema_version",
					"phase",
					"result"
				],
				properties: {
					schema_version: {
						type: "integer",
						const: 1
					},
					phase: {
						type: "string",
						const: "paths"
					},
					result: {}
				}
			},
			{
				type: "object",
				additionalProperties: !1,
				required: [
					"schema_version",
					"phase",
					"result"
				],
				properties: {
					schema_version: {
						type: "integer",
						const: 1
					},
					phase: {
						type: "string",
						const: "comparison"
					},
					result: {}
				}
			},
			{
				type: "object",
				additionalProperties: !1,
				required: [
					"schema_version",
					"phase",
					"error"
				],
				properties: {
					schema_version: {
						type: "integer",
						const: 1
					},
					phase: {
						type: "string",
						enum: ["paths", "comparison"]
					},
					error: {
						type: "string",
						minLength: 1
					}
				}
			}
		]
	}, r = t().default;
	function i(e, { instancePath: t = "", parentData: a, parentDataProperty: o, rootData: s = e, dynamicAnchors: c = {} } = {}) {
		let l = null, u = 0, d = i.evaluated;
		d.dynamicProps && (d.props = void 0), d.dynamicItems && (d.items = void 0);
		let f = u, p = !1, m = null, h = u;
		if (u === h) {
			if (e && typeof e == "object" && !Array.isArray(e)) {
				let n;
				if (e.schema_version === void 0 && (n = "schema_version") || e.phase === void 0 && (n = "phase") || e.result === void 0 && (n = "result")) {
					let e = {
						instancePath: t,
						schemaPath: "#/oneOf/0/required",
						keyword: "required",
						params: { missingProperty: n },
						message: "must have required property '" + n + "'"
					};
					l === null ? l = [e] : l.push(e), u++;
				} else {
					let n = u;
					for (let n in e) if (n !== "schema_version" && n !== "phase" && n !== "result") {
						let e = {
							instancePath: t,
							schemaPath: "#/oneOf/0/additionalProperties",
							keyword: "additionalProperties",
							params: { additionalProperty: n },
							message: "must NOT have additional properties"
						};
						l === null ? l = [e] : l.push(e), u++;
						break;
					}
					if (n === u) {
						if (e.schema_version !== void 0) {
							let n = e.schema_version, r = u;
							if (typeof n != "number" || n % 1 || isNaN(n) || !isFinite(n)) {
								let e = {
									instancePath: t + "/schema_version",
									schemaPath: "#/oneOf/0/properties/schema_version/type",
									keyword: "type",
									params: { type: "integer" },
									message: "must be integer"
								};
								l === null ? l = [e] : l.push(e), u++;
							}
							if (n !== 1) {
								let e = {
									instancePath: t + "/schema_version",
									schemaPath: "#/oneOf/0/properties/schema_version/const",
									keyword: "const",
									params: { allowedValue: 1 },
									message: "must be equal to constant"
								};
								l === null ? l = [e] : l.push(e), u++;
							}
							var g = r === u;
						} else var g = !0;
						if (g) {
							if (e.phase !== void 0) {
								let n = e.phase, r = u;
								if (typeof n != "string") {
									let e = {
										instancePath: t + "/phase",
										schemaPath: "#/oneOf/0/properties/phase/type",
										keyword: "type",
										params: { type: "string" },
										message: "must be string"
									};
									l === null ? l = [e] : l.push(e), u++;
								}
								if (n !== "paths") {
									let e = {
										instancePath: t + "/phase",
										schemaPath: "#/oneOf/0/properties/phase/const",
										keyword: "const",
										params: { allowedValue: "paths" },
										message: "must be equal to constant"
									};
									l === null ? l = [e] : l.push(e), u++;
								}
								var g = r === u;
							} else var g = !0;
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
				l === null ? l = [e] : l.push(e), u++;
			}
		}
		var _ = h === u;
		if (_) {
			p = !0, m = 0;
			var v = !0;
		}
		let y = u;
		if (u === y) {
			if (e && typeof e == "object" && !Array.isArray(e)) {
				let n;
				if (e.schema_version === void 0 && (n = "schema_version") || e.phase === void 0 && (n = "phase") || e.result === void 0 && (n = "result")) {
					let e = {
						instancePath: t,
						schemaPath: "#/oneOf/1/required",
						keyword: "required",
						params: { missingProperty: n },
						message: "must have required property '" + n + "'"
					};
					l === null ? l = [e] : l.push(e), u++;
				} else {
					let n = u;
					for (let n in e) if (n !== "schema_version" && n !== "phase" && n !== "result") {
						let e = {
							instancePath: t,
							schemaPath: "#/oneOf/1/additionalProperties",
							keyword: "additionalProperties",
							params: { additionalProperty: n },
							message: "must NOT have additional properties"
						};
						l === null ? l = [e] : l.push(e), u++;
						break;
					}
					if (n === u) {
						if (e.schema_version !== void 0) {
							let n = e.schema_version, r = u;
							if (typeof n != "number" || n % 1 || isNaN(n) || !isFinite(n)) {
								let e = {
									instancePath: t + "/schema_version",
									schemaPath: "#/oneOf/1/properties/schema_version/type",
									keyword: "type",
									params: { type: "integer" },
									message: "must be integer"
								};
								l === null ? l = [e] : l.push(e), u++;
							}
							if (n !== 1) {
								let e = {
									instancePath: t + "/schema_version",
									schemaPath: "#/oneOf/1/properties/schema_version/const",
									keyword: "const",
									params: { allowedValue: 1 },
									message: "must be equal to constant"
								};
								l === null ? l = [e] : l.push(e), u++;
							}
							var b = r === u;
						} else var b = !0;
						if (b) {
							if (e.phase !== void 0) {
								let n = e.phase, r = u;
								if (typeof n != "string") {
									let e = {
										instancePath: t + "/phase",
										schemaPath: "#/oneOf/1/properties/phase/type",
										keyword: "type",
										params: { type: "string" },
										message: "must be string"
									};
									l === null ? l = [e] : l.push(e), u++;
								}
								if (n !== "comparison") {
									let e = {
										instancePath: t + "/phase",
										schemaPath: "#/oneOf/1/properties/phase/const",
										keyword: "const",
										params: { allowedValue: "comparison" },
										message: "must be equal to constant"
									};
									l === null ? l = [e] : l.push(e), u++;
								}
								var b = r === u;
							} else var b = !0;
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
				l === null ? l = [e] : l.push(e), u++;
			}
		}
		var _ = y === u;
		if (_ && p) p = !1, m = [m, 1];
		else {
			_ && (p = !0, m = 1, v !== !0 && (v = !0));
			let i = u;
			if (u === i) {
				if (e && typeof e == "object" && !Array.isArray(e)) {
					let i;
					if (e.schema_version === void 0 && (i = "schema_version") || e.phase === void 0 && (i = "phase") || e.error === void 0 && (i = "error")) {
						let e = {
							instancePath: t,
							schemaPath: "#/oneOf/2/required",
							keyword: "required",
							params: { missingProperty: i },
							message: "must have required property '" + i + "'"
						};
						l === null ? l = [e] : l.push(e), u++;
					} else {
						let i = u;
						for (let n in e) if (n !== "schema_version" && n !== "phase" && n !== "error") {
							let e = {
								instancePath: t,
								schemaPath: "#/oneOf/2/additionalProperties",
								keyword: "additionalProperties",
								params: { additionalProperty: n },
								message: "must NOT have additional properties"
							};
							l === null ? l = [e] : l.push(e), u++;
							break;
						}
						if (i === u) {
							if (e.schema_version !== void 0) {
								let n = e.schema_version, r = u;
								if (typeof n != "number" || n % 1 || isNaN(n) || !isFinite(n)) {
									let e = {
										instancePath: t + "/schema_version",
										schemaPath: "#/oneOf/2/properties/schema_version/type",
										keyword: "type",
										params: { type: "integer" },
										message: "must be integer"
									};
									l === null ? l = [e] : l.push(e), u++;
								}
								if (n !== 1) {
									let e = {
										instancePath: t + "/schema_version",
										schemaPath: "#/oneOf/2/properties/schema_version/const",
										keyword: "const",
										params: { allowedValue: 1 },
										message: "must be equal to constant"
									};
									l === null ? l = [e] : l.push(e), u++;
								}
								var x = r === u;
							} else var x = !0;
							if (x) {
								if (e.phase !== void 0) {
									let r = e.phase, i = u;
									if (typeof r != "string") {
										let e = {
											instancePath: t + "/phase",
											schemaPath: "#/oneOf/2/properties/phase/type",
											keyword: "type",
											params: { type: "string" },
											message: "must be string"
										};
										l === null ? l = [e] : l.push(e), u++;
									}
									if (r !== "paths" && r !== "comparison") {
										let e = {
											instancePath: t + "/phase",
											schemaPath: "#/oneOf/2/properties/phase/enum",
											keyword: "enum",
											params: { allowedValues: n.oneOf[2].properties.phase.enum },
											message: "must be equal to one of the allowed values"
										};
										l === null ? l = [e] : l.push(e), u++;
									}
									var x = i === u;
								} else var x = !0;
								if (x) {
									if (e.error !== void 0) {
										let n = e.error, i = u;
										if (u === i) {
											if (typeof n == "string") {
												if (r(n) < 1) {
													let e = {
														instancePath: t + "/error",
														schemaPath: "#/oneOf/2/properties/error/minLength",
														keyword: "minLength",
														params: { limit: 1 },
														message: "must NOT have fewer than 1 characters"
													};
													l === null ? l = [e] : l.push(e), u++;
												}
											} else {
												let e = {
													instancePath: t + "/error",
													schemaPath: "#/oneOf/2/properties/error/type",
													keyword: "type",
													params: { type: "string" },
													message: "must be string"
												};
												l === null ? l = [e] : l.push(e), u++;
											}
										}
										var x = i === u;
									} else var x = !0;
								}
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
					l === null ? l = [e] : l.push(e), u++;
				}
			}
			var _ = i === u;
			_ && p ? (p = !1, m = [m, 2]) : _ && (p = !0, m = 2, v !== !0 && (v = !0));
		}
		if (p) u = f, l !== null && (f ? l.length = f : l = null);
		else {
			let e = {
				instancePath: t,
				schemaPath: "#/oneOf",
				keyword: "oneOf",
				params: { passingSchemas: m },
				message: "must match exactly one schema in oneOf"
			};
			return l === null ? l = [e] : l.push(e), u++, i.errors = l, !1;
		}
		return i.errors = l, d.props = v, u === 0;
	}
	i.evaluated = {
		dynamicProps: !0,
		dynamicItems: !1
	};
})))().validatePathReplyEnvelope;
export { n as validatePathReplyEnvelope };
