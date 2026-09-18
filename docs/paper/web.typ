// Web presentation only; all definitions, proofs, and examples stay in reductions.typ.
#let export-details = sys.inputs.at("details", default: "false") == "true"

#let detail-key(name, variant) = {
  if variant == none { return name }
  name + "/" + variant.keys().sorted().map(key => key + "=" + str(variant.at(key))).join(",")
}

#let detail-article(key, body) = {
  let anchor = key.replace("problem:", "def:").replace("rule:", "thm:").replace("->", "-to-")
  [#html.elem("article", attrs: ("data-detail-key": key), body)#label(anchor)]
}
