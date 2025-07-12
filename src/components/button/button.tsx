import "./button.css";

interface ButtonProps {
  text?: string;
  icon?: string;
  onClick?: () => void;
}

function Button({ text, icon, onClick }: ButtonProps) {
  return (
    <div className="button-container">
      {icon != null ? <img src={icon}></img> : null}
      <button onClick={onClick}>{text}</button>
    </div>
  );
}

export default Button;
