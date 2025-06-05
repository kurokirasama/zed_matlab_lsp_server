; indents.scm

; De-indent when encountering 'end' for various blocks
(function_definition (end) @end) @indent
(if_statement (end) @end) @indent
(switch_statement (end) @end) @indent
(for_statement (end) @end) @indent
(while_statement (end) @end) @indent
(try_statement (end) @end) @indent

; Class blocks
(class_definition (end) @end) @indent
(properties_block (end) @end) @indent
(methods_block (end) @end) @indent
(enumeration_block (end) @end) @indent
(events_block (end) @end) @indent

; Note: More complex indentation rules (e.g., inside parentheses, for line continuations)
; might require additional patterns depending on the grammar structure and desired behavior.
; This provides a basic starting point for blocks closed by 'end'.