{% import "macros.hpp" as macros  %}
// This file is generated. Do not edit.

#include "Win32Api.hpp"

Win32Api::Win32Api() {
}

Win32Api::~Win32Api() {
}

{% for function in functions %}
{{ macros::cxx_type(type = function.return_type) }} Win32Api::{{- function.name -}}({{- macros::cxx_arguments_with_names(args = function.params) -}}) {
    return ::{{ function.name }}({{- function.params | map(attribute='name') | join(sep = ', ') -}});
}
{% endfor %}
