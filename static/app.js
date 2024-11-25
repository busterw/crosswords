// Function to display the list of crosswords as a carousel
function displayCrosswords(crosswords) {
    let currentIndex = 0;
    const gridContainer = document.getElementById('gridContainer');
    const previousButton = document.getElementById('previousButton');
    const nextButton = document.getElementById('nextButton');
    const totalCrosswords = crosswords.length;

    // Function to display a crossword at the current index
    function displayCrosswordAtIndex(index) {
        const crossword = crosswords[index];
        const grid = crossword.preview;

        // Clear previous grid
        gridContainer.innerHTML = "";

        // Dynamically set the grid-template-columns based on the crossword size
        const numColumns = grid[0].length;
        gridContainer.style.gridTemplateColumns = `repeat(${numColumns}, 30px)`;
        gridContainer.style.gridAutoRows = `${30}px`; // Set fixed row height (same as grid cell height)

        // Iterate through the grid and render each cell
        grid.forEach((row, rowIndex) => {
            const rowDiv = document.createElement('div');
            rowDiv.classList.add('grid-row');
            
            row.forEach((cell, colIndex) => {
                const cellDiv = document.createElement('div');
                cellDiv.classList.add('grid-cell');

                if (cell === 0) {
                    // Black square (empty space)
                    cellDiv.classList.add('black');
                } else {
                    // White square (with letter)
                    cellDiv.classList.add('white');
                    const span = document.createElement('span');
                    span.textContent = cell; // Display the character in the cell
                    cellDiv.appendChild(span);
                }
                rowDiv.appendChild(cellDiv);
            });
            gridContainer.appendChild(rowDiv);
        });
    }

    // Initialize the display with the first crossword
    displayCrosswordAtIndex(currentIndex);

    // Function to handle the previous button
    function showPreviousCrossword() {
        if (currentIndex > 0) {
            currentIndex--;
            displayCrosswordAtIndex(currentIndex);
        }
    }

    // Function to handle the next button
    function showNextCrossword() {
        if (currentIndex < totalCrosswords - 1) {
            currentIndex++;
            displayCrosswordAtIndex(currentIndex);
        }
    }

    // Attach the event listeners for the buttons
    previousButton.addEventListener('click', showPreviousCrossword);
    nextButton.addEventListener('click', showNextCrossword);
}

// Function to handle the form submission
document.getElementById('generateButton').addEventListener('click', async function() {
    const wordsInput = document.getElementById('wordsInput');
    const words = wordsInput.value.split(',').map(word => word.trim());

    if (words.length > 0) {
        try {
            // Send request to the backend
            const response = await fetch('/submit-words', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ words: words })
            });

            if (response.ok) {
                const crosswords = await response.json();
                console.log("Crosswords data received:", crosswords);

                // Display the crosswords
                displayCrosswords(crosswords);
            } else {
                console.error('Error fetching crosswords:', response.statusText);
            }
        } catch (error) {
            console.error('Error in fetch request:', error);
        }
    }
});
