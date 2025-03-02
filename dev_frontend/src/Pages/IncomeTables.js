import { utils } from 'near-api-js';
import React, { useState, useEffect } from 'react';
import Nav from '../Nav';
import { getListOfIncomeTables } from '../utils';


export default function IncomeTables() {
  const [incomeTables, setIncomeTables] = useState([]);

  // Get the list of IncomeTables
  useEffect(async () => {
    const list = await getListOfIncomeTables(0, 1000);
    setIncomeTables(list);
  }, []);


  return (
    <>
      <Nav />
      <main>
        <h2>Payout a Song</h2>

        <section>
          {"This is the list:"}
          <ul className="revenueList">
            {incomeTables.map((IncomeTable) => (
              <li className="incomeTableEntry" key={IncomeTable[0]}>
                <p><strong>{"TreeIndex: "}{IncomeTable[0]}</strong></p>
                <p><i>{"Contract: "}</i>{IncomeTable[1].contract}</p>
                <p><i>{"Root ID: "}</i>{IncomeTable[1].root_id}</p>
                <p><i>{"Owner: "}</i>{IncomeTable[1].owner}</p>
                <p><i>{"Current Balance: "}</i>
                  {(IncomeTable[1].current_balance === 0) ? "0" : utils.format.formatNearAmount(BigInt(IncomeTable[1].current_balance), 2)}{" NEAR"}
                </p>
                <p><i>{"Total Income: "}</i>
                  {(IncomeTable[1].total_income === 0) ? "0" : utils.format.formatNearAmount(BigInt(IncomeTable[1].total_income), 2)}{" NEAR"}
                </p>
              </li>
            ))}
          </ul>
        </section>
      </main>
    </>
  )
}
