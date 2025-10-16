{% import "templates/macros.hpp" as macros  %}
// This file is generated. Do not edit.
#pragma once

#include "IWin32Api.hpp"

class Win32Api : public IWin32Api {
public:
    Win32Api();
    virtual ~Win32Api();
    
    {% for function in functions %}
    {{ macros::cxx_type(type = function.return_type) }} {{ function.name -}}({{- macros::cxx_arguments_with_names(args = function.params) -}}) override;
    {% endfor %}
};
