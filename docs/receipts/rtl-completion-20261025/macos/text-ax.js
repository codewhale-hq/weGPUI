function run(argv) {
 const p=Application('System Events').processes.whose({unixId:Number(argv[0])})[0];
 const out=[];
 function get(e,k){try{return e.attributes.byName(k).value();}catch(err){return null;}}
 function walk(e,d){
  if(d>40||out.length)return;
  const role=get(e,'AXRole');
  if(role==='AXTextArea'){
   const row={};
   for(const k of ['AXRole','AXTitle','AXDescription','AXValue','AXSelectedText','AXSelectedTextRange','AXNumberOfCharacters','AXFocused'])row[k]=get(e,k);
   out.push(row);
  }
  let n=0;try{n=e.uiElements.length;}catch(err){}
  for(let i=0;i<n;i++)walk(e.uiElements[i],d+1);
 }
 walk(p.windows[0],0);return JSON.stringify(out);
}
