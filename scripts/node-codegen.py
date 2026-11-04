"""Shared generation helpers for language-neutral operation declarations."""
import re

def closing(text, start):
    depth=0;quoted=False;escaped=False
    for pos in range(start,len(text)):
        ch=text[pos]
        if quoted:
            if escaped:escaped=False
            elif ch=='\\':escaped=True
            elif ch=='"':quoted=False
            continue
        if ch=='"':quoted=True
        elif ch=='(':depth+=1
        elif ch==')':
            depth-=1
            if depth==0:return pos
    raise ValueError('unbalanced generated operation')

def operations(source):
    """Render the same typed Rust operation calls for the owned Node adapters."""
    source=re.sub(r'#\[(?:pymethods|pyo3)(?:\([^\]]*\))?\]\s*','',source)
    source=re.sub(r"py: Python<'py>,?\s*",'',source)
    source=source.replace("<'py>",'')
    source=re.sub(r"Bound<'(?:py|_), PyAny>", "Input", source)
    source=re.sub(r'\bpy,\s*','',source).replace('(py)','()')
    source=source.replace('PyResult','Result').replace('-> Result<Input>','-> Result<Output>')
    source=re.sub(r'(?m)^(    )fn ',r'\1pub(crate) async fn ',source)
    for match in reversed(list(re.finditer(r'\.run\(',source))):
        end=closing(source,match.end()-1);source=source[:end+1]+'.await'+source[end+1:]
    return source
