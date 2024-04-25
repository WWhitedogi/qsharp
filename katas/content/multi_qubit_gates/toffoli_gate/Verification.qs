namespace Kata.Verification {
    open Microsoft.Quantum.Katas;
    open Microsoft.Quantum.Intrinsic;

    operation ToffoliGate (qs : Qubit[]) : Unit is Adj + Ctl {
        CCNOT(qs[0], qs[1], qs[2]);
    }

    operation CheckSolution() : Bool {
        let solution = Kata.ToffoliGate;
        let reference = ToffoliGate;
        let isCorrect = CheckOperationsEquivalenceStrict(solution, reference, 3);

        // Output different feedback to the user depending on whether the solution was correct.
        if isCorrect {
            Message("Correct!");
        } else {
            Message("Incorrect.");
            Message("Hint: examine the state prepared by your solution and compare it with the state it " +
                "is expected to prepare.");
            ShowQuantumStateComparison(3, (qs => ()), solution, reference);
        }

        isCorrect
    }
}