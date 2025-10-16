{% import "templates/macros.hpp" as macros  %}
// This file is generated. Do not edit.
#pragma once

#include <Windows.h>
#include <cstdint>
#include <cstddef>

{% for alias, type in type_aliases %}
using {{ alias }} = {{ type }};
{% endfor %}

class IWin32Api {
public:
    {% for function in functions %}
    virtual {{ macros::cxx_type(type = function.return_type) }} {{ function.name -}}({{- macros::cxx_arguments_with_names(args = function.params) -}}) = 0;
    {% endfor %}
};
