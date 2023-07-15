import {fireEvent, render} from "@testing-library/react";
import TestComponent, {TestComponentProps} from "../TestComponent";

describe('testing out jest', function () {
  test('sum', () => {
    expect(1+2).toBe(3)
  })
});


function renderTestComponent(props: Partial<TestComponentProps> = {}) {
  const defaultProps: TestComponentProps = {
    numToShow: 3,
    onClick: () => {
      // do nothing
    },
  }
  return render(<TestComponent {...defaultProps} {...props}/>)
}
describe('<TestComponent/>', function () {
  test('should display the number', async () => {
    const queryMethods = renderTestComponent({numToShow: 5})

    const paragraph = await queryMethods.findByTestId("paragraph")

    expect(paragraph).toHaveTextContent("5")
  })

  test('should call callback on click', async () => {
    const onClickTest = jest.fn()
    const queryMethods = renderTestComponent({onClick: onClickTest})
    const clickContainer = await queryMethods.findByTestId("container")
    fireEvent.click(clickContainer)
    expect(onClickTest).toBeCalled()
  })
});