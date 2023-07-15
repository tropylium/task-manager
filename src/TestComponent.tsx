import React from "react";
// import "../styles/TestComponent.css"

export interface TestComponentProps {
  numToShow: number,
  onClick: () => void
}

function TestComponent({numToShow, onClick}: TestComponentProps): React.ReactElement {
  return (
    <div data-testid="container" onClick={onClick}>
        <p data-testid="paragraph">{numToShow}</p>
    </div>
  );
}

export default TestComponent;