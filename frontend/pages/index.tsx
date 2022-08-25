import axios from "axios"
import { GetStaticProps } from "next"
import { BarChart, LineChart, TreeMapChart } from "../components/Charts"
import { BACKEND_URL } from "./consts"
import { averageDaily, convertToBarChart, convertToLine, convertToTreemap, formatNumberToINR } from "./utils"


export const getStaticProps: GetStaticProps = async () => {
  const monthlyExpenses = await convertToLine(`${BACKEND_URL}/monthly/Expenses`, "Expenses")
  const monthlyIncome = await convertToLine(`${BACKEND_URL}/monthly/Income`, "Income", -1)

  const cashflowExpenses = await convertToLine(`${BACKEND_URL}/cashflow/Expenses`, "Expenses")
  const cashflowIncome = await convertToLine(`${BACKEND_URL}/cashflow/Income`, "Income", -1)

  const montlhyChart = await convertToBarChart(`${BACKEND_URL}/monthly/Assets?convert_commodity=true`)
  const expenses = await convertToTreemap(`${BACKEND_URL}/split/Expenses`, "Expenses")

  const { data: assets } = await axios.get(`${BACKEND_URL}/balance/Assets?convert_commodity=true`)
  const { data: liability } = await axios.get(`${BACKEND_URL}/balance/Liability?convert_commodity=true`)
  const avgDailyExpenses = await averageDaily("Expenses")
  const avgDailyIncome = await averageDaily("Income", -1)

  const netWorth = {
    amount: assets.amount - liability.amount,
    account: "Assets:NetWorth"
  }


  return {
    props: {
      monthly: [
        monthlyExpenses, monthlyIncome
      ],
      cashflow: [
        cashflowExpenses, cashflowIncome
      ],
      montlhyChart,
      expenses,
      balance: [
        netWorth,
        avgDailyExpenses,
        avgDailyIncome,
      ]
    }
  }
}

const Index = ({ monthly, cashflow, montlhyChart, expenses, balance }) => {
  return (
    <div className="container">
      <div className="float">
        {balance.map((bal, index) =>
          <div key={index} className="box">
            <h6>Your {bal.account.split(":").pop()} Is </h6>
            <h3>{formatNumberToINR(bal.amount)}</h3>
          </div>
        )}
      </div>

      <h4> Monthly Income and Expenses</h4>
      <LineChart data={monthly} />
      <h4> Cashflow Of Income and Expenses</h4>
      <LineChart data={cashflow} />
      <h4> Monthly Spendings </h4>
      <BarChart data={montlhyChart} />
      <h4> Expenses Distrubution </h4>
      <TreeMapChart data={expenses} />
    </div>
  )
}

export default Index
