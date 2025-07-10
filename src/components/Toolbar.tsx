import Button from "./Button.tsx"

const style = {
  'display': 'flex',
  'width': 'fit-content',
  'height': 'fit-content',
  'flex-direction': 'row',
  'align-items': 'center',
  'justify-content': 'center',
  'gap': '10px',
  'margin': '20px auto',
  'padding': '10px',
  'background-color': '#f6f6f6',
  'border-radius': '10px',
  'border-style': 'solid',
  'border-width': '1px',
  'pointer-events': 'auto',
};


function Toolbar(props) {
  return (
      <div style={style}>
        <Button text="1"/>
        <Button text="2"/>
        <Button text="3"/>
      </div>
  );
}

export default Toolbar;
